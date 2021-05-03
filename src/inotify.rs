/* dnsmasq is Copyright (c) 2000-2021 Simon Kelley
 
   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; version 2 dated June, 1991, or
   (at your option) version 3 dated 29 June, 2007.
 
   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.
     
   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/


 HAVE_INOTIFY

#include <sys/inotify.h>
#include <sys/param.h> /* For MAXSYMLINKS */

/* the strategy is to set an inotify on the directories containing
   resolv files, for any files in the directory which are close-write 
   or moved into the directory.
   
   When either of those happen, we look to see if the file involved
   is actually a resolv-file, and if so, call poll-resolv with
   the "force" argument, to ensure it's read.

   This adds one new error condition: the directories containing
   all specified resolv-files must exist at start-up, even if the actual
   files don't. 
*/

 char *inotify_buffer;
#define INOTIFY_SZ (sizeof(struct inotify_event) + NAME_MAX + 1)

/* If path is a symbolic link, return the path it
   points to, made absolute if relative.
   If path doesn't exist or is not a symlink, return NULL.
   Return value is malloc'ed */
 char *my_readlink(path: &mut String)
{
  src: usize, size = 64;
  char *buf;

  while (1)
    {
      buf = safe_malloc(size);
      rc = readlink(path, buf, (size_t)size);
      
      if (rc == -1)
	{
	  /* Not link or doesn't exist. */
	  if (errno == EINVAL || errno == ENOENT)
	    {
	      free(buf);
	      return NULL;
	    }
	  else
	    die(format!("cannot access path {}: {}"), path, EC_MISC);
	}
      else if (rc < size-1)
	{
	  char *d;
	  
	  buf[rc] = 0;
	  if (buf[0] != '/' && ((d = strrchr(path, '/'))))
	    {
	      /* Add path to relative link */
	      char *new_buf = safe_malloc((d - path) + strlen(buf) + 2);
	      *(d+1) = 0;
	      strcpy(new_buf, path);
	      strcat(new_buf, buf);
	      free(buf);
	      buf = new_buf;
	    }
	  return buf;
	}

      /* Buffer too small, increase and retry */
      size += 64;
      free(buf);
    }
}

void inotify_dnsmasq_init()
{
  let mut res: resolvc;
  inotify_buffer = safe_malloc(INOTIFY_SZ);
  daemon.inotifyfd = inotify_init1(IN_NONBLOCK | IN_CLOEXEC);
  
  if (daemon.inotifyfd == -1)
    die(format!("failed to create inotify: {}"), NULL, EC_MISC);

  if (daemon.opt_no_resolv)
    return;
  
  for (res = daemon.resolv_files; res; res = res.next)
    {
      d: &mut String, *new_path, *path = safe_malloc(strlen(res.name) + 1);
      int links = MAXSYMLINKS;

      strcpy(path, res.name);

      /* Follow symlinks until we reach a non-symlink, or a non-existent file. */
      while ((new_path = my_readlink(path)))
	{
	  if (links-- == 0)
	    die(format!("too many symlinks following {}"), res.name, EC_MISC);
	  free(path);
	  path = new_path;
	}

      res.wd = -1;

      if ((d = strrchr(path, '/')))
	{
	  *d = 0; /* make path just directory */
	  res.wd = inotify_add_watch(daemon.inotifyfd, path, IN_CLOSE_WRITE | IN_MOVED_TO);

	  res.file = d+1; /* pointer to filename */
	  *d = '/';
	  
	  if (res.wd == -1 && errno == ENOENT)
	    die(format!("directory {} for resolv-file is missing, cannot poll"), res.name, EC_MISC);
	}	  
	 
      if (res.wd == -1)
	die(format!("failed to create inotify for {}: {}"), res.name, EC_MISC);
	
    }
}


/* initialisation for dynamic-dir. Set inotify watch for each directory, and read pre-existing files */
void set_dynamic_inotify(flag: i32, total_size: i32, struct crec **rhash, revhashsz: i32)
{
  let mut ah: hostsfile;
  
  for (ah = daemon.dynamic_dirs; ah; ah = ah.next)
    {
      DIR *dir_stream = NULL;
      let mut ent: dirent;
      struct stat buf;
     
      if (!(ah.flags & flag))
	continue;
 
      if (stat(ah.fname, &buf) == -1 || !(S_ISDIR(buf.st_mode)))
	{
	  my_syslog(LOG_ERR, format!("bad dynamic directory {}: {}"), 
		    ah.fname, strerror(errno));
	  continue;
	}
      
       if (!(ah.flags & AH_WD_DONE))
	 {
	   ah.wd = inotify_add_watch(daemon.inotifyfd, ah.fname, IN_CLOSE_WRITE | IN_MOVED_TO);
	   ah.flags |= AH_WD_DONE;
	 }

       /* Read contents of dir _after_ calling add_watch, in the hope of avoiding
	  a race which misses files being added as we start */
       if (ah.wd == -1 || !(dir_stream = opendir(ah.fname)))
	 {
	   my_syslog(LOG_ERR, format!("failed to create inotify for {}: {}"),
		     ah.fname, strerror(errno));
	   continue;
	 }

       while ((ent = readdir(dir_stream)))
	 {
	   lendir: usize = strlen(ah.fname);
	   lenfile: usize = strlen(ent.d_name);
	   char *path;
	   
	   /* ignore emacs backups and dotfiles */
	   if (lenfile == 0 || 
	       ent.d_name[lenfile - 1] == '~' ||
	       (ent.d_name[0] == '#' && ent.d_name[lenfile - 1] == '#') ||
	       ent.d_name[0] == '.')
	     continue;
	   
	   if ((path = whine_malloc(lendir + lenfile + 2)))
	     {
	       strcpy(path, ah.fname);
	       strcat(path, "/");
	       strcat(path, ent.d_name);
	       
	       /* ignore non-regular files */
	       if (stat(path, &buf) != -1 && S_ISREG(buf.st_mode))
		 {
		   if (ah.flags & AH_HOSTS)
		     total_size = read_hostsfile(path, ah.index, total_size, rhash, revhashsz);
 
		   else if (ah.flags & (AH_DHCP_HST | AH_DHCP_OPT))
		     option_read_dynfile(path, ah.flags);
		   
		 }

	       free(path);
	     }
	 }

       closedir(dir_stream);
    }
}

int inotify_check(now: time::Instant)
{
  let mut hit: i32 = 0;
  let mut ah: hostsfile;

  while (1)
    {
      let mut rc: i32;
      char *p;
      let mut res: resolvc;
      let mut in: inotify_event;

      while ((rc = read(daemon.inotifyfd, inotify_buffer, INOTIFY_SZ)) == -1 && errno == EINTR);
      
      if (rc <= 0)
	break;
      
      for (p = inotify_buffer; rc - (p - inotify_buffer) >= sizeof(struct inotify_event); p += sizeof(struct inotify_event) + in.len) 
	{
	  namelen: usize;

	  in = (struct inotify_event*)p;
	  
	  /* ignore emacs backups and dotfiles */
	  if (in.len == 0 || (namelen = strlen(in.name)) == 0 ||
	      in.name[namelen - 1] == '~' ||
	      (in.name[0] == '#' && in.name[namelen - 1] == '#') ||
	      in.name[0] == '.')
	    continue;

	  for (res = daemon.resolv_files; res; res = res.next)
	    if (res.wd == in.wd && strcmp(res.file, in.name) == 0)
	      hit = 1;

	  for (ah = daemon.dynamic_dirs; ah; ah = ah.next)
	    if (ah.wd == in.wd)
	      {
		lendir: usize = strlen(ah.fname);
		char *path;
		
		if ((path = whine_malloc(lendir + in.len + 2)))
		  {
		    strcpy(path, ah.fname);
		    strcat(path, "/");
		    strcat(path, in.name);
		     
		    my_syslog(LOG_INFO, format!("inotify, new or changed file {}"), path);

		    if (ah.flags & AH_HOSTS)
		      {
			read_hostsfile(path, ah.index, 0, NULL, 0);
 
			if (daemon.dhcp || daemon.doing_dhcp6) 
			  {
			    /* Propagate the consequences of loading a new dhcp-host */
			    dhcp_update_configs(daemon.dhcp_conf);
			    lease_update_from_configs(); 
			    lease_update_file(now); 
			    lease_update_dns(1);
			  }

		      }
 
		    else if (ah.flags & AH_DHCP_HST)
		      {
			if (option_read_dynfile(path, AH_DHCP_HST))
			  {
			    /* Propagate the consequences of loading a new dhcp-host */
			    dhcp_update_configs(daemon.dhcp_conf);
			    lease_update_from_configs(); 
			    lease_update_file(now); 
			    lease_update_dns(1);
			  }
		      }
		    else if (ah.flags & AH_DHCP_OPT)
		      option_read_dynfile(path, AH_DHCP_OPT);

		    
		    free(path);
		  }
	      }
	}
    }
  return hit;
}

  /* INOTIFY */
  
