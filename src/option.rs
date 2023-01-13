

/* This is too insanely large to keep in-line in the switch */
use crate::dhcp_opt::DhcpOpt;
use crate::dnsmasq_defines::{DHOPT_ENCAPSULATE, DHOPT_MATCH, DHOPT_RFC3925, DHOPT_VENDOR, OT_INTERNAL};

pub fn parse_dhcp_opt(errstr: &mut String, arg: &mut String, flags: i32) -> i32
{
  let mut new = DhcpOpt::default();
  let mut len= 0u8;
    let mut cp = String::new();
    let mut addrs = 0i32;
    let mut digs = 0i32;
    let mut is_addr = 0i32;
    let mut is_addr6 = 0i32;
    let mut is_hex = 0i32;
    let mut is_dec = 0i32;
    let mut is_string = 0i32;
    let mut dots = 0i32;
    let mut comma = String::new();
    let mut opt_len = 0u16;
    let mut is6 = 0i32;
    let mut option_ok = 0i32;

  new.len = 0;
  new.flags = flags;
  new.netid = NULL;
  new.val = NULL;
  new.opt = 0;
  
  while arg
    {
      comma = split(arg);      

      for (cp = arg; *cp; cp++)
	if *cp < '0' || *cp > '9'
	  break;
      
      if !*cp
	{
	  new.opt = atoi(arg);
	  opt_len = 0;
	  option_ok = 1;
	  break;
	}
      
      if strstr(arg, "option:") == arg
	{
	  if (new.opt = lookup_dhcp_opt(AF_INET, arg+7)) != -1
	    {
	      opt_len = lookup_dhcp_len(AF_INET, new.opt);
	      /* option:<optname> must follow tag and vendor string. */
	      if !(opt_len & OT_INTERNAL) || flags == DHOPT_MATCH
		option_ok = 1;
	    }
	  break;
	}
// #ifdef HAVE_DHCP6
      else if strstr(arg, "option6:") == arg
	{
	  for (cp = arg+8; *cp; cp++)
	    if *cp < '0' || *cp > '9'
	      break;
	 
	  if !*cp
	    {
	      new.opt = atoi(arg+8);
	      opt_len = 0;
	      option_ok = 1;
	    }
	  else
	    {
	      if (new.opt = lookup_dhcp_opt(AF_INET6, arg+8)) != -1
		{
		  opt_len = lookup_dhcp_len(AF_INET6, new.opt);
		  if !(opt_len & OT_INTERNAL) || flags == DHOPT_MATCH
		    option_ok = 1;
		}
	    }
	  /* option6:<opt>|<optname> must follow tag and vendor string. */
	  is6 = 1;
	  break;
	}
// #endif
      else if strstr(arg, "vendor:") == arg
	{
	  new.u.vendor_class = (unsigned char *)opt_string_alloc(arg+7);
	  new.flags |= DHOPT_VENDOR;
	  // if ((new.flags & DHOPT_ENCAPSULATE) || flags == DHOPT_MATCH)
	    goto_err(_("inappropriate vendor:"));
	}
      else if strstr(arg, "encap:") == arg
	{
	  new.u.encap = atoi(arg+6);
	  new.flags |= DHOPT_ENCAPSULATE;
	  if (new.flags & DHOPT_VENDOR) || flags == DHOPT_MATCH
	    goto_err(_("inappropriate encap:"));
	}
      else if strstr(arg, "vi-encap:") == arg
	{
	  new.u.encap = atoi(arg+9);
	  new.flags |= DHOPT_RFC3925;
	  if flags == DHOPT_MATCH
	    {
	      option_ok = 1;
	      break;
	    }
	}
      else
	{
	  /* allow optional "net:" or "tag:" for consistency */
	  const char *name = (is_tag_prefix(arg)) ? arg+4 : set_prefix(arg);
	  new.netid = dhcp_netid_create(name, new.netid);
	}
      
      arg = comma; 
    }

// #ifdef HAVE_DHCP6
  if is6
    {
      if new.flags & (DHOPT_VENDOR | DHOPT_ENCAPSULATE) {
          goto_err(_("unsupported encapsulation for IPv6 option"));
      }
      
      if opt_len == 0 &&
	  !(new.flags & DHOPT_RFC3925) {
          opt_len = lookup_dhcp_len(AF_INET6, new.opt);
      }
    }
  else {
// #endif
      if opt_len == 0 && !(new.flags & (DHOPT_VENDOR | DHOPT_ENCAPSULATE | DHOPT_RFC3925)) {
          opt_len = lookup_dhcp_len(AF_INET, new.opt);
      }
  }
  
  /* option may be missing with rfc3925 match */
  if (!option_ok) {
      goto_err(_("bad dhcp-option"));
  }
  
  if comma
    {
      /* characterise the value */
      char c;
      int found_dig = 0, found_colon = 0;
      is_addr = is_addr6 = is_hex = is_dec = is_string = 1;
      addrs = digs = 1;
      dots = 0;
      for (cp = comma; (c = *cp); cp++)
	if c == ','
	  {
	    addrs++;
	    is_dec = is_hex = 0;
	  }
	else if (c == ':')
	  {
	    digs++;
	    is_dec = is_addr = 0;
	    found_colon = 1;
	  }
	else if (c == '/') 
	  {
	    is_addr6 = is_dec = is_hex = 0;
	    if (cp == comma) /* leading / means a pathname */
	      is_addr = 0;
	  } 
	else if (c == '.')	
	  {
	    is_dec = is_hex = 0;
	    dots++;
	  }
	else if (c == '-')
	  is_hex = is_addr = is_addr6 = 0;
	else if (c == ' ')
	  is_dec = is_hex = 0;
	else if (!(c >='0' && c <= '9'))
	  {
	    is_addr = 0;
	    if (cp[1] == 0 && is_dec &&
		(c == 'b' || c == 's' || c == 'i'))
	      {
		lenchar = c;
		*cp = 0;
	      }
	    else
	      is_dec = 0;
	    if (!((c >='A' && c <= 'F') ||
		  (c >='a' && c <= 'f') || 
		  (c == '*' && (flags & DHOPT_MATCH))))
	      {
		is_hex = 0;
		if (c != '[' && c != ']')
		  is_addr6 = 0;
	      }
	  }
	else
	  found_dig = 1;
     
      if (!found_dig)
	is_dec = is_addr = 0;

      if (!found_colon)
	is_addr6 = 0;

// #ifdef HAVE_DHCP6
      /* NTP server option takes hex, addresses or FQDN */
      if (is6 && new.opt == OPTION6_NTP_SERVER && !is_hex)
	opt_len |= is_addr6 ? OT_ADDR_LIST : OT_RFC1035_NAME;
// #endif
     
      /* We know that some options take addresses */
      if (opt_len & OT_ADDR_LIST)
	{
	  is_string = is_dec = is_hex = 0;
	  
	  if (!is6 && (!is_addr || dots == 0))
	    goto_err(_("bad IP address"));

	   if (is6 && !is_addr6)
	     goto_err(_("bad IPv6 address"));
	}
      /* or names */
      else if (opt_len & (OT_NAME | OT_RFC1035_NAME | OT_CSTRING))
	is_addr6 = is_addr = is_dec = is_hex = 0;
      
      if (found_dig && (opt_len & OT_TIME) && strlen(comma) > 0)
	{
	  int val, fac = 1;

	  switch (comma[strlen(comma) - 1])
	    {
	    case 'w':
	    case 'W':
	      fac *= 7;
	      /* fall through */
	    case 'd':
	    case 'D':
	      fac *= 24;
	      /* fall through */
	    case 'h':
	    case 'H':
	      fac *= 60;
	      /* fall through */
	    case 'm':
	    case 'M':
	      fac *= 60;
	      /* fall through */
	    case 's':
	    case 'S':
	      comma[strlen(comma) - 1] = 0;
	    }
	  
	  new.len = 4;
	  new.val = opt_malloc(4);
	  val = atoi(comma);
	  *((int *)new.val) = htonl(val * fac);
	}  
      else if (is_hex && digs > 1)
	{
	  new.len = digs;
	  new.val = opt_malloc(new.len);
	  parse_hex(comma, new.val, digs, (flags & DHOPT_MATCH) ? &new.u.wildcard_mask : NULL, NULL);
	  new.flags |= DHOPT_HEX;
	}
      else if (is_dec)
	{
	  int i, val = atoi(comma);
	  /* assume numeric arg is 1 byte except for
	     options where it is known otherwise.
	     For vendor class option, we have to hack. */
	  if (opt_len != 0)
	    new.len = opt_len;
	  else if (val & 0xffff0000)
	    new.len = 4;
	  else if (val & 0xff00)
	    new.len = 2;
	  else
	    new.len = 1;

	  if (lenchar == 'b')
	    new.len = 1;
	  else if (lenchar == 's')
	    new.len = 2;
	  else if (lenchar == 'i')
	    new.len = 4;
	  
	  new.val = opt_malloc(new.len);
	  for (i=0; i<new.len; i++)
	    new.val[i] = val>>((new.len - i - 1)*8);
	}
      else if (is_addr && !is6)	
	{
	  in: in_addr;
	  unsigned char *op;
	  char *slash;
	  /* max length of address/subnet descriptor is five bytes,
	     add one for the option 120 enc byte too */
	  new.val = op = opt_malloc((5 * addrs) + 1);
	  new.flags |= DHOPT_ADDR;

	  if (!(new.flags & (DHOPT_ENCAPSULATE | DHOPT_VENDOR | DHOPT_RFC3925)) &&
	      new.opt == OPTION_SIP_SERVER)
	    {
	      *(op++) = 1; /* RFC 3361 "enc byte" */
	      new.flags &= ~DHOPT_ADDR;
	    }
	  while (addrs--) 
	    {
	      cp = comma;
	      comma = split(cp);
	      slash = split_chr(cp, '/');
	      if (!inet_pton(AF_INET, cp, &in))
		goto_err(_("bad IPv4 address"));
	      if (!slash)
		{
		  memcpy(op, &in, INADDRSZ);
		  op += INADDRSZ;
		}
	      else
		{
		  unsigned char *p = (unsigned char *)&in;
		  int netsize = atoi(slash);
		  *op++ = netsize;
		  if (netsize > 0)
		    *op++ = *p++;
		  if (netsize > 8)
		    *op++ = *p++;
		  if (netsize > 16)
		    *op++ = *p++;
		  if (netsize > 24)
		    *op++ = *p++;
		  new.flags &= ~DHOPT_ADDR; /* cannot re-write descriptor format */
		} 
	    }
	  new.len = op - new.val;
	}
      else if (is_addr6 && is6)
	{
	  unsigned char *op;
	  new.val = op = opt_malloc(16 * addrs);
	  new.flags |= DHOPT_ADDR6;
	  while (addrs--) 
	    {
	      cp = comma;
	      comma = split(cp);
	      
	      /* check for [1234::7] */
	      if (*cp == '[')
		cp++;
	      if (strlen(cp) > 1 && cp[strlen(cp)-1] == ']')
		cp[strlen(cp)-1] = 0;
	      
	      if (inet_pton(AF_INET6, cp, op))
		{
		  op += IN6ADDRSZ;
		  continue;
		}

	      goto_err(_("bad IPv6 address"));
	    } 
	  new.len = op - new.val;
	}
      else if (is_string)
	{
 	  /* text arg */
	  if ((new.opt == OPTION_DOMAIN_SEARCH || new.opt == OPTION_SIP_SERVER) &&
	      !is6 && !(new.flags & (DHOPT_ENCAPSULATE | DHOPT_VENDOR | DHOPT_RFC3925)))
	    {
	      /* dns search, RFC 3397, or SIP, RFC 3361 */
	      q: *mut u8 *r, *tail;
	      p: *mut u8 *m = NULL, *newp;
	      size_t newlen, len = 0;
	      int header_size = (new.opt == OPTION_DOMAIN_SEARCH) ? 0 : 1;
	      
	      arg = comma;
	      comma = split(arg);
	      
	      while (arg && *arg)
		{
		  char *in, *dom = NULL;
		  size_t domlen = 1;
		  /* Allow "." as an empty domain */
		  if (strcmp (arg, ".") != 0)
		    {
		      if (!(dom = canonicalise_opt(arg)))
			goto_err(_("bad domain in dhcp-option"));
			
		      domlen = strlen(dom) + 2;
		    }
		      
		  newp = opt_malloc(len + domlen + header_size);
		  if (m)
		    {
		      memcpy(newp, m, header_size + len);
		      free(m);
		    }
		  m = newp;
		  p = m + header_size;
		  q = p + len;
		  
		  /* add string on the end in RFC1035 format */
		  for (in = dom; in && *in;) 
		    {
		      unsigned char *cp = q++;
		      j: i32;
		      for (j = 0; *in && (*in != '.'); in++, j++)
			*q++ = *in;
		      *cp = j;
		      if (*in)
			in++;
		    }
		  *q++ = 0;
		  free(dom);
		  
		  /* Now tail-compress using earlier names. */
		  newlen = q - p;
		  for (tail = p + len; *tail; tail += (*tail) + 1)
		    for (r = p; r - p < (int)len; r += (*r) + 1)
		      if (strcmp((char *)r, (char *)tail) == 0)
			{
			  PUTSHORT((r - p) | 0xc000, tail); 
			  newlen = tail - p;
			  goto end;
			}
		end:
		  len = newlen;
		  
		  arg = comma;
		  comma = split(arg);
		}
      
	      /* RFC 3361, enc byte is zero for names */
	      if (new.opt == OPTION_SIP_SERVER && m)
		m[0] = 0;
	      new.len = (int) len + header_size;
	      new.val = m;
	    }
// #ifdef HAVE_DHCP6
	  else if (comma && (opt_len & OT_CSTRING))
	    {
	      /* length fields are two bytes so need 16 bits for each string */
	      int i, commas = 1;
	      p: *mut u8 *newp;

	      for (i = 0; comma[i]; i++)
		if (comma[i] == ',')
		  commas++;
	      
	      newp = opt_malloc(strlen(comma)+(2*commas));	  
	      p = newp;
	      arg = comma;
	      comma = split(arg);
	      
	      while (arg && *arg)
		{
		  u16 len = strlen(arg);
		  unhide_metas(arg);
		  PUTSHORT(len, p);
		  memcpy(p, arg, len);
		  p += len; 

		  arg = comma;
		  comma = split(arg);
		}

	      new.val = newp;
	      new.len = p - newp;
	    }
	  else if (comma && (opt_len & OT_RFC1035_NAME))
	    {
	      unsigned char *p = NULL, *q, *newp, *end;
	      int len = 0;
	      int header_size = (is6 && new.opt == OPTION6_NTP_SERVER) ? 4 : 0;
	      arg = comma;
	      comma = split(arg);
	      
	      while (arg && *arg)
		{
		  char *dom = canonicalise_opt(arg);
		  if (!dom)
		    goto_err(_("bad domain in dhcp-option"));
		    		  
		  newp = opt_malloc(len + header_size + strlen(dom) + 2);
		  
		  if (p)
		    {
		      memcpy(newp, p, len);
		      free(p);
		    }
		  
		  p = newp;
		  q = p + len;
		  end = do_rfc1035_name(q + header_size, dom, NULL);
		  *end++ = 0;
		  if (is6 && new.opt == OPTION6_NTP_SERVER)
		    {
		      PUTSHORT(NTP_SUBOPTION_SRV_FQDN, q);
		      PUTSHORT(end - q - 2, q);
		    }
		  len = end - p;
		  free(dom);

		  arg = comma;
		  comma = split(arg);
		}
	      
	      new.val = p;
	      new.len = len;
	    }
// #endif
	  else
	    {
	      new.len = strlen(comma);
	      /* keep terminating zero on string */
	      new.val = (unsigned char *)opt_string_alloc(comma);
	      new.flags |= DHOPT_STRING;
	    }
	}
    }

  if (!is6 && 
      ((new.len > 255) ||
      (new.len > 253 && (new.flags & (DHOPT_VENDOR | DHOPT_ENCAPSULATE))) ||
       (new.len > 250 && (new.flags & DHOPT_RFC3925))))
    goto_err(_("dhcp-option too long"));
  
  if (flags == DHOPT_MATCH)
    {
      if ((new.flags & (DHOPT_ENCAPSULATE | DHOPT_VENDOR)) ||
	  !new.netid ||
	  new.netid.next)
	goto_err(_("illegal dhcp-match"));
       
      if (is6)
	{
	  new.next = daemon.dhcp_match6;
	  daemon.dhcp_match6 = new;
	}
      else
	{
	  new.next = daemon.dhcp_match;
	  daemon.dhcp_match = new;
	}
    }
  else if (is6)
    {
      new.next = daemon.dhcp_opts6;
      daemon.dhcp_opts6 = new;
    }
  else
    {
      new.next = daemon.dhcp_opts;
      daemon.dhcp_opts = new;
    }
    
  return 1;
on_error:
  dhcp_opt_free(new);
  return 0;
}

// #endif

void set_option_bool(unsigned int opt)
{
  option_var(opt) |= option_val(opt);
}

void reset_option_bool(unsigned int opt)
{
  option_var(opt) &= ~(option_val(opt));
}

static int one_opt(int option, char *arg, char *errstr, char *gen_err, int command_line, int servers_only)
{      
  i: i32;
  char *comma;

  if (option == '?')
    ret_err(gen_err);
  
  for (i=0; usage[i].opt != 0; i++)
    if (usage[i].opt == option)
      {
	 int rept = usage[i].rept;
	 
	 if (command_line)
	   {
	     /* command line */
	     if (rept == ARG_USED_CL)
	       ret_err(_("illegal repeated flag"));
	     if (rept == ARG_ONE)
	       usage[i].rept = ARG_USED_CL;
	   }
	 else
	   {
	     /* allow file to override command line */
	     if (rept == ARG_USED_FILE)
	       ret_err(_("illegal repeated keyword"));
	     if (rept == ARG_USED_CL || rept == ARG_ONE)
	       usage[i].rept = ARG_USED_FILE;
	   }

	 if (rept != ARG_DUP && rept != ARG_ONE && rept != ARG_USED_CL) 
	   {
	     set_option_bool(rept);
	     return 1;
	   }
       
	 break;
      }
  
  switch (option)
    { 
    case 'C': /* --conf-file */
      {
	char *file = opt_string_alloc(arg);
	if (file)
	  {
	    one_file(file, 0);
	    free(file);
	  }
	break;
      }

    case LOPT_CONF_SCRIPT: /* --conf-script */
      {
	char *file = opt_string_alloc(arg);
	if (file)
	  {
	    one_file(file, LOPT_CONF_SCRIPT);
	    free(file);
	  }
	break;
      }

    case '7': /* --conf-dir */	      
      {
	DIR *dir_stream;
	struct dirent *ent;
	char *directory, *path;
	struct list {
	  char *name;
	  struct list *next;
	} *ignore_suffix = NULL, *match_suffix = NULL, *files = NULL, *li;
	
	comma = split(arg);
	if (!(directory = opt_string_alloc(arg)))
	  break;
	
	for (arg = comma; arg; arg = comma) 
	  {
	    comma = split(arg);
	    if (strlen(arg) != 0)
	      {
		li = opt_malloc(sizeof(struct list));
		if (*arg == '*')
		  {
		    /* "*" with no suffix is a no-op */
		    if (arg[1] == 0)
		      free(li);
		    else
		      {
			li.next = match_suffix;
			match_suffix = li;
			/* Have to copy: buffer is overwritten */
			li.name = opt_string_alloc(arg+1);
		      }
		  }
		else
		  {
		    li.next = ignore_suffix;
		    ignore_suffix = li;
		    /* Have to copy: buffer is overwritten */
		    li.name = opt_string_alloc(arg);
		  }
	      }
	  }
	
	if (!(dir_stream = opendir(directory)))
	  die(_("cannot access directory %s: %s"), directory, EC_FILE);
	
	while ((ent = readdir(dir_stream)))
	  {
	    size_t len = strlen(ent.d_name);
	    struct stat buf;
	    
	    /* ignore emacs backups and dotfiles */
	    if (len == 0 ||
		ent.d_name[len - 1] == '~' ||
		(ent.d_name[0] == '#' && ent.d_name[len - 1] == '#') ||
		ent.d_name[0] == '.')
	      continue;

	    if (match_suffix)
	      {
		for (li = match_suffix; li; li = li.next)
		  {
		    /* check for required suffices */
		    size_t ls = strlen(li.name);
		    if (len > ls &&
			strcmp(li.name, &ent.d_name[len - ls]) == 0)
		      break;
		  }
		if (!li)
		  continue;
	      }
	    
	    for (li = ignore_suffix; li; li = li.next)
	      {
		/* check for proscribed suffices */
		size_t ls = strlen(li.name);
		if (len > ls &&
		    strcmp(li.name, &ent.d_name[len - ls]) == 0)
		  break;
	      }
	    if (li)
	      continue;
	    
	    path = opt_malloc(strlen(directory) + len + 2);
	    strcpy(path, directory);
	    strcat(path, "/");
	    strcat(path, ent.d_name);

	    /* files must be readable */
	    if (stat(path, &buf) == -1)
	      die(_("cannot access %s: %s"), path, EC_FILE);
	    
	    /* only reg files allowed. */
	    if (S_ISREG(buf.st_mode))
	      {
		/* sort files into order. */
		struct list **up, *new = opt_malloc(sizeof(struct list));
		new.name = path;
		
		for (up = &files, li = files; li; up = &li.next, li = li.next)
		  if (strcmp(li.name, path) >=0)
		    break;

		new.next = li;
		*up = new;
	      }
	    else
	      free(path);

	  }

	for (li = files; li; li = li.next)
	  one_file(li.name, 0);
      	
	closedir(dir_stream);
	free(directory);
	for(; ignore_suffix; ignore_suffix = li)
	  {
	    li = ignore_suffix.next;
	    free(ignore_suffix.name);
	    free(ignore_suffix);
	  }
	for(; match_suffix; match_suffix = li)
	  {
	    li = match_suffix.next;
	    free(match_suffix.name);
	    free(match_suffix);
	  }
	for(; files; files = li)
	  {
	    li = files.next;
	    free(files.name);
	    free(files);
	  }
	break;
      }

    case LOPT_ADD_SBNET: /* --add-subnet */
      set_option_bool(OPT_CLIENT_SUBNET);
      if (arg)
	{
          char *err, *end;
	  comma = split(arg);

          struct mysubnet* new = opt_malloc(sizeof(struct mysubnet));
          if ((end = split_chr(arg, '/')))
	    {
	      /* has subnet+len */
	      err = parse_mysockaddr(arg, &new.addr);
	      if (err)
		ret_err_free(err, new);
	      if (!atoi_check(end, &new.mask))
		ret_err_free(gen_err, new);
	      new.addr_used = 1;
	    } 
	  else if (!atoi_check(arg, &new.mask))
	    ret_err_free(gen_err, new);
	    
          daemon.add_subnet4 = new;

          if (comma)
            {
	      new = opt_malloc(sizeof(struct mysubnet));
	      if ((end = split_chr(comma, '/')))
		{
		  /* has subnet+len */
                  err = parse_mysockaddr(comma, &new.addr);
                  if (err)
                    ret_err_free(err, new);
                  if (!atoi_check(end, &new.mask))
                    ret_err_free(gen_err, new);
                  new.addr_used = 1;
                }
              else
                {
                  if (!atoi_check(comma, &new.mask))
                    ret_err_free(gen_err, new);
                }
          
	      daemon.add_subnet6 = new;
	    }
	}
      break;

    case '1': /* --enable-dbus */
      set_option_bool(OPT_DBUS);
      if (arg)
	daemon.dbus_name = opt_string_alloc(arg);
      else
	daemon.dbus_name = DNSMASQ_SERVICE;
      break;

    case LOPT_UBUS: /* --enable-ubus */
      set_option_bool(OPT_UBUS);
      if (arg)
	daemon.ubus_name = opt_string_alloc(arg);
      else
	daemon.ubus_name = DNSMASQ_UBUS_NAME;
      break;

    case '8': /* --log-facility */
      /* may be a filename */
      if (strchr(arg, '/') || strcmp (arg, "-") == 0)
	daemon.log_file = opt_string_alloc(arg);
      else
	{	  
// #ifdef __ANDROID__
	  ret_err(_("setting log facility is not possible under Android"));
#else
	  for (i = 0; facilitynames[i].c_name; i++)
	    if (hostname_isequal((char *)facilitynames[i].c_name, arg))
	      break;
	  
	  if (facilitynames[i].c_name)
	    daemon.log_fac = facilitynames[i].c_val;
	  else
	    ret_err(_("bad log facility"));
// #endif
	}
      break;

    case 'x': /* --pid-file */
      daemon.runfile = opt_string_alloc(arg);
      break;

    case 'r': /* --resolv-file */
      {
	char *name = opt_string_alloc(arg);
	struct resolvc *new, *list = daemon.resolv_files;
	
	if (list && list.is_default)
	  {
	    /* replace default resolv file - possibly with nothing */
	    if (name)
	      {
		list.is_default = 0;
		list.name = name;
	      }
	    else
	      list = NULL;
	  }
	else if (name)
	  {
	    new = opt_malloc(sizeof(struct resolvc));
	    new.next = list;
	    new.name = name;
	    new.is_default = 0;
	    new.mtime = 0;
	    new.logged = 0;
	    list = new;
	  }
	daemon.resolv_files = list;
	break;
      }

    case LOPT_SERVERS_FILE:
      daemon.servers_file = opt_string_alloc(arg);
      break;
      
    case 'm':  /* --mx-host */
      {
	int pref = 1;
	struct mx_srv_record *new;
	char *name, *target = NULL;

	if ((comma = split(arg)))
	  {
	    char *prefstr;
	    if ((prefstr = split(comma)) && !atoi_check16(prefstr, &pref))
	      ret_err(_("bad MX preference"));
	  }
	
	if (!(name = canonicalise_opt(arg)) || 
	    (comma && !(target = canonicalise_opt(comma))))
	  {
	    free(name);
	    free(target);
	    ret_err(_("bad MX name"));
	  }
	
	new = opt_malloc(sizeof(struct mx_srv_record));
	new.next = daemon.mxnames;
	daemon.mxnames = new;
	new.issrv = 0;
	new.name = name;
	new.target = target; /* may be NULL */
	new.weight = pref;
	break;
      }
      
    case 't': /*  --mx-target */
      if (!(daemon.mxtarget = canonicalise_opt(arg)))
	ret_err(_("bad MX target"));
      break;

    case LOPT_DUMPFILE:  /* --dumpfile */
      daemon.dump_file = opt_string_alloc(arg);
      break;

    case LOPT_DUMPMASK:  /* --dumpmask */
      daemon.dump_mask = strtol(arg, NULL, 0);
      break;
      
// #ifdef HAVE_DHCP
    case 'l':  /* --dhcp-leasefile */
      daemon.lease_file = opt_string_alloc(arg);
      break;
      
      /* Sorry about the gross pre-processor abuse */
    case '6':             /* --dhcp-script */
    case LOPT_LUASCRIPT:  /* --dhcp-luascript */
#  if !defined(HAVE_SCRIPT)
      ret_err(_("recompile with HAVE_SCRIPT defined to enable lease-change scripts"));
#  else
      if (option == LOPT_LUASCRIPT)
#    if !defined(HAVE_LUASCRIPT)
	ret_err(_("recompile with HAVE_LUASCRIPT defined to enable Lua scripts"));
#    else
        daemon.luascript = opt_string_alloc(arg);
#    endif
      else
        daemon.lease_change_command = opt_string_alloc(arg);
#  endif
      break;
// #endif /* HAVE_DHCP */

    case LOPT_DHCP_HOST:     /* --dhcp-hostsfile */
    case LOPT_DHCP_OPTS:     /* --dhcp-optsfile */
    case 'H':                /* --addn-hosts */
      {
	struct hostsfile *new = opt_malloc(sizeof(struct hostsfile));
	new.fname = opt_string_alloc(arg);
	new.index = daemon.host_index++;
	new.flags = 0;
	if (option == 'H')
	  {
	    new.next = daemon.addn_hosts;
	    daemon.addn_hosts = new;
	  }
	else if (option == LOPT_DHCP_HOST)
	  {
	    new.next = daemon.dhcp_hosts_file;
	    daemon.dhcp_hosts_file = new;
	  }
	else if (option == LOPT_DHCP_OPTS)
	  {
	    new.next = daemon.dhcp_opts_file;
	    daemon.dhcp_opts_file = new;
	  }
	
	break;
      }

    case LOPT_DHCP_INOTIFY:  /* --dhcp-hostsdir */
    case LOPT_DHOPT_INOTIFY: /* --dhcp-optsdir */
    case LOPT_HOST_INOTIFY:  /* --hostsdir */
      {
	struct dyndir *new = opt_malloc(sizeof(struct dyndir));
	new.dname = opt_string_alloc(arg);
	new.flags = 0;
	new.next = daemon.dynamic_dirs;
	daemon.dynamic_dirs = new;
	if (option == LOPT_DHCP_INOTIFY)
	new.flags |= AH_DHCP_HST;
	else if (option == LOPT_DHOPT_INOTIFY)
	new.flags |= AH_DHCP_OPT;
	else if (option == LOPT_HOST_INOTIFY)
	new.flags |= AH_HOSTS;

	break;
      }
      
    case LOPT_AUTHSERV: /* --auth-server */
      comma = split(arg);
      
      daemon.authserver = opt_string_alloc(arg);
      
      while ((arg = comma))
	{
	  struct iname *new = opt_malloc(sizeof(struct iname));
	  comma = split(arg);
	  new.name = NULL;
	  unhide_metas(arg);
	  if (inet_pton(AF_INET, arg, &new.addr.in.sin_addr) > 0)
	    new.addr.sa.sa_family = AF_INET;
	  else if (inet_pton(AF_INET6, arg, &new.addr.in6.sin6_addr) > 0)
	    new.addr.sa.sa_family = AF_INET6;
	  else
	    {
	      char *fam = split_chr(arg, '/');
	      new.name = opt_string_alloc(arg);
	      new.addr.sa.sa_family = 0;
	      if (fam)
		{
		  if (strcmp(fam, "4") == 0)
		    new.addr.sa.sa_family = AF_INET;
		  else if (strcmp(fam, "6") == 0)
		    new.addr.sa.sa_family = AF_INET6;
		  else
		  {
		    free(new.name);
		    ret_err_free(gen_err, new);
		  }
		} 
	    }
	  new.next = daemon.authinterface;
	  daemon.authinterface = new;
	};
            
      break;

    case LOPT_AUTHSFS: /* --auth-sec-servers */
      {
	struct name_list *new;

	do {
	  comma = split(arg);
	  new = opt_malloc(sizeof(struct name_list));
	  new.name = opt_string_alloc(arg);
	  new.next = daemon.secondary_forward_server;
	  daemon.secondary_forward_server = new;
	  arg = comma;
	} while (arg);
	break;
      }
	
    case LOPT_AUTHZONE: /* --auth-zone */
      {
	struct auth_zone *new;
	
	comma = split(arg);
		
	new = opt_malloc(sizeof(struct auth_zone));
	new.domain = canonicalise_opt(arg);
	if (!new.domain)
	  ret_err_free(_("invalid auth-zone"), new);
 	new.subnet = NULL;
	new.exclude = NULL;
	new.interface_names = NULL;
	new.next = daemon.auth_zones;
	daemon.auth_zones = new;

	while ((arg = comma))
	  {
	    int prefixlen = 0;
	    int is_exclude = 0;
	    char *prefix;
	    struct addrlist *subnet =  NULL;
	    union all_addr addr;

	    comma = split(arg);
	    prefix = split_chr(arg, '/');
	    
	    if (prefix && !atoi_check(prefix, &prefixlen))
	      ret_err(gen_err);
	    
	    if (strstr(arg, "exclude:") == arg)
	      {
		    is_exclude = 1;
		    arg = arg+8;
	      }

	    if (inet_pton(AF_INET, arg, &addr.addr4))
	      {
		subnet = opt_malloc(sizeof(struct addrlist));
		subnet.prefixlen = (prefixlen == 0) ? 24 : prefixlen;
		subnet.flags = ADDRLIST_LITERAL;
	      }
	    else if (inet_pton(AF_INET6, arg, &addr.addr6))
	      {
		subnet = opt_malloc(sizeof(struct addrlist));
		subnet.prefixlen = (prefixlen == 0) ? 64 : prefixlen;
		subnet.flags = ADDRLIST_LITERAL | ADDRLIST_IPV6;
	      }
	    else 
	      {
		struct auth_name_list *name =  opt_malloc(sizeof(struct auth_name_list));
		name.name = opt_string_alloc(arg);
		name.flags = AUTH4 | AUTH6;
		name.next = new.interface_names;
		new.interface_names = name;
		if (prefix)
		  {
		    if (prefixlen == 4)
		      name.flags &= ~AUTH6;
		    else if (prefixlen == 6)
		      name.flags &= ~AUTH4;
		    else
		      ret_err(gen_err);
		  }
	      }
	    
	    if (subnet)
	      {
		subnet.addr = addr;

		if (is_exclude)
		  {
		    subnet.next = new.exclude;
		    new.exclude = subnet;
		  }
		else
		  {
		    subnet.next = new.subnet;
		    new.subnet = subnet;
		  }
	      }
	  }
	break;
      }
      
    case  LOPT_AUTHSOA: /* --auth-soa */
      comma = split(arg);
      daemon.soa_sn = (u32)atoi(arg);
      if (comma)
	{
	  char *cp;
	  arg = comma;
	  comma = split(arg);
	  daemon.hostmaster = opt_string_alloc(arg);
	  for (cp = daemon.hostmaster; cp && *cp; cp++)
	    if (*cp == '@')
	      *cp = '.';

	  if (comma)
	    {
	      arg = comma;
	      comma = split(arg); 
	      daemon.soa_refresh = (u32)atoi(arg);
	      if (comma)
		{
		  arg = comma;
		  comma = split(arg); 
		  daemon.soa_retry = (u32)atoi(arg);
		  if (comma)
		    daemon.soa_expiry = (u32)atoi(comma);
		}
	    }
	}

      break;

    case 's':         /* --domain */
    case LOPT_SYNTH:  /* --synth-domain */
      if (strcmp (arg, "#") == 0)
	set_option_bool(OPT_RESOLV_DOMAIN);
      else
	{
	  char *d, *d_raw = arg;
	  comma = split(arg);
	  if (!(d = canonicalise_opt(d_raw)))
	    ret_err(gen_err);
	  else
	    {
	      free(d); /* allocate this again below. */
	      if (comma)
		{
		  struct cond_domain *new = opt_malloc(sizeof(struct cond_domain));
		  char *netpart;
		  
		  new.prefix = NULL;
		  new.indexed = 0;
		  new.prefixlen = 0;
		  
		  unhide_metas(comma);
		  if ((netpart = split_chr(comma, '/')))
		    {
		      msize: i32;

		      arg = split(netpart);
		      if (!atoi_check(netpart, &msize))
			ret_err_free(gen_err, new);
		      else if (inet_pton(AF_INET, comma, &new.start))
			{
			  mask: i32;

			  if (msize > 32)
			     ret_err_free(_("bad prefix length"), new);
			  
			  mask = (1 << (32 - msize)) - 1;
			  new.is6 = 0;
			  new.start.s_addr = ntohl(htonl(new.start.s_addr) & ~mask);
			  new.end.s_addr = new.start.s_addr | htonl(mask);
			  if (arg)
			    {
			      if (option != 's')
				{
				  if (!(new.prefix = canonicalise_opt(arg)) ||
				      strlen(new.prefix) > MAXLABEL - INET_ADDRSTRLEN)
				    ret_err_free(_("bad prefix"), new);
				}
			      else if (strcmp(arg, "local") != 0)
				ret_err_free(gen_err, new);
			      else
				{
				  /* local=/xxx.yyy.zzz.in-addr.arpa/ */
				  domain_rev4(0, NULL, &new.start, msize);
				 				  
				  /* local=/<domain>/ */
				  /* d_raw can't failed to canonicalise here, checked above. */
				  add_update_server(SERV_LITERAL_ADDRESS, NULL, NULL, NULL, d_raw, NULL);
				}
			    }
			}
		      else if (inet_pton(AF_INET6, comma, &new.start6))
			{
			  u64 mask, addrpart = addr6part(&new.start6);

			  if (msize > 128)
			    ret_err_free(_("bad prefix length"), new);

			  mask = (1LLU << (128 - msize)) - 1LLU;

			  new.is6 = 1;
			  new.prefixlen = msize;
			  
			  /* prefix==64 overflows the mask calculation above */
			  if (msize <= 64)
			    mask = (u64)-1LL;
			  
			  new.end6 = new.start6;
			  setaddr6part(&new.start6, addrpart & ~mask);
			  setaddr6part(&new.end6, addrpart | mask);
			  
			  if (arg)
			    {
			      if (option != 's')
				{
				  if (!(new.prefix = canonicalise_opt(arg)) ||
				      strlen(new.prefix) > MAXLABEL - INET6_ADDRSTRLEN)
				    ret_err_free(_("bad prefix"), new);
				}	
			      else if (strcmp(arg, "local") != 0)
				ret_err_free(gen_err, new);
			      else 
				{
				  /* generate the equivalent of
				     local=/xxx.yyy.zzz.ip6.arpa/ */
				  domain_rev6(0, NULL, &new.start6, msize);
				  
				  /* local=/<domain>/ */
				  /* d_raw can't failed to canonicalise here, checked above. */
				  add_update_server(SERV_LITERAL_ADDRESS, NULL, NULL, NULL, d_raw, NULL);
				}
			    }
			}
		      else
			ret_err_free(gen_err, new);
		    }
		  else
		    {
		      char *prefstr;
		      arg = split(comma);
		      prefstr = split(arg);

		      if (inet_pton(AF_INET, comma, &new.start))
			{
			  new.is6 = 0;
			  if (!arg)
			    new.end.s_addr = new.start.s_addr;
			  else if (!inet_pton(AF_INET, arg, &new.end))
			    ret_err_free(gen_err, new);
			}
		      else if (inet_pton(AF_INET6, comma, &new.start6))
			{
			  new.is6 = 1;
			  if (!arg)
			    memcpy(&new.end6, &new.start6, IN6ADDRSZ);
			  else if (!inet_pton(AF_INET6, arg, &new.end6))
			    ret_err_free(gen_err, new);
			}
		      else if (option == 's')
			{
			  /* subnet from interface. */
			  new.interface = opt_string_alloc(comma);
			  new.al = NULL;
			}
		      else
			ret_err_free(gen_err, new);
		      
		      if (option != 's' && prefstr)
			{
			  if (!(new.prefix = canonicalise_opt(prefstr)) ||
			      strlen(new.prefix) > MAXLABEL - INET_ADDRSTRLEN)
			    ret_err_free(_("bad prefix"), new);
			}
		    }

		  new.domain = canonicalise_opt(d_raw);
		  if (option  == 's')
		    {
		      new.next = daemon.cond_domain;
		      daemon.cond_domain = new;
		    }
		  else
		    {
		      char *star;
		      if (new.prefix &&
			  (star = strrchr(new.prefix, '*'))
			  && *(star+1) == 0)
			{
			  *star = 0;
			  new.indexed = 1;
			  if (new.is6 && new.prefixlen < 64)
			    ret_err_free(_("prefix length too small"), new);
			}
		      new.next = daemon.synth_domains;
		      daemon.synth_domains = new;
		    }
		}
	      else if (option == 's')
		daemon.domain_suffix = canonicalise_opt(d_raw);
	      else 
		ret_err(gen_err);
	    }
	}
      break;
      
    case LOPT_CPE_ID: /* --add-dns-client */
      if (arg)
	daemon.dns_client_id = opt_string_alloc(arg);
      break;

    case LOPT_UMBRELLA: /* --umbrella */
      set_option_bool(OPT_UMBRELLA);
      while (arg)
	{
	  comma = split(arg);
	  if (strstr(arg, "deviceid:"))
	    {
	      char *p;
	      u8 *u = daemon.umbrella_device;
	      word: [u8;3]
	      
	      arg += 9;
	      if (strlen(arg) != 16)
		ret_err(gen_err);
	      
	      for (p = arg; *p; p++)
		if (!isxdigit((int)*p))
		  ret_err(gen_err);
	      
	      set_option_bool(OPT_UMBRELLA_DEVID);
	      
	      for (i = 0; i < (int)sizeof(daemon.umbrella_device); i++, arg+=2)
		{
		  memcpy(word, &(arg[0]), 2);
		  *u++ = strtoul(word, NULL, 16);
		}
	    }
	  else if (strstr(arg, "orgid:"))
	    {
	      if (!strtoul_check(arg+6, &daemon.umbrella_org))
		ret_err(gen_err);
	    }
	  else if (strstr(arg, "assetid:"))
	    {
	      if (!strtoul_check(arg+8, &daemon.umbrella_asset))
		ret_err(gen_err);
	    }
	  else
	    ret_err(gen_err);
	  
	  arg = comma;
	}
      break;
      
    case LOPT_ADD_MAC: /* --add-mac */
      if (!arg)
	set_option_bool(OPT_ADD_MAC);
      else
	{
	  unhide_metas(arg);
	  if (strcmp(arg, "base64") == 0)
	    set_option_bool(OPT_MAC_B64);
	  else if (strcmp(arg, "text") == 0)
	    set_option_bool(OPT_MAC_HEX);
	  else
	    ret_err(gen_err);
	}
      break;

    case 'u':  /* --user */
      daemon.username = opt_string_alloc(arg);
      break;
      
    case 'g':  /* --group */
      daemon.groupname = opt_string_alloc(arg);
      daemon.group_set = 1;
      break;

// #ifdef HAVE_DHCP
    case LOPT_SCRIPTUSR: /* --scriptuser */
      daemon.scriptuser = opt_string_alloc(arg);
      break;
// #endif
      
    case 'i':  /* --interface */
      do {
	struct iname *new = opt_malloc(sizeof(struct iname));
	comma = split(arg);
	new.next = daemon.if_names;
	daemon.if_names = new;
	/* new.name may be NULL if someone does
	   "interface=" to disable all interfaces except loop. */
	new.name = opt_string_alloc(arg);
	new.used = 0;
	arg = comma;
      } while (arg);
      break;
      
    case LOPT_TFTP: /* --enable-tftp */
      set_option_bool(OPT_TFTP);
      if (!arg)
	break;
      /* fall through */

    case 'I':  /* --except-interface */
    case '2':  /* --no-dhcp-interface */
      do {
	struct iname *new = opt_malloc(sizeof(struct iname));
	comma = split(arg);
	new.name = opt_string_alloc(arg);
	if (option == 'I')
	  {
	    new.next = daemon.if_except;
	    daemon.if_except = new;
	  }
	else if (option == LOPT_TFTP)
	   {
	    new.next = daemon.tftp_interfaces;
	    daemon.tftp_interfaces = new;
	  }
	else
	  {
	    new.next = daemon.dhcp_except;
	    daemon.dhcp_except = new;
	  }
	arg = comma;
      } while (arg);
      break;
      
    case 'B':  /* --bogus-nxdomain */
    case LOPT_IGNORE_ADDR: /* --ignore-address */
     {
	union all_addr addr;
	int prefix, is6 = 0;
	struct bogus_addr *baddr;
	
	unhide_metas(arg);

	if (!arg ||
	    ((comma = split_chr(arg, '/')) && !atoi_check(comma, &prefix)))
	  ret_err(gen_err);

	if (inet_pton(AF_INET6, arg, &addr.addr6) == 1)
	  is6 = 1;
	else if (inet_pton(AF_INET, arg, &addr.addr4) != 1)
	  ret_err(gen_err);

	if (!comma)
	  {
	    if (is6)
	      prefix = 128;
	    else
	      prefix = 32;
	  }

	if (prefix > 128 || (!is6 && prefix > 32))
	  ret_err(gen_err);
	
	baddr = opt_malloc(sizeof(struct bogus_addr));
	if (option == 'B')
	  {
	    baddr.next = daemon.bogus_addr;
	    daemon.bogus_addr = baddr;
	  }
	else
	  {
	    baddr.next = daemon.ignore_addr;
	    daemon.ignore_addr = baddr;
	  }

	baddr.prefix = prefix;
	baddr.is6 = is6;
	baddr.addr = addr;
	break;
     }
      
    case 'a':  /* --listen-address */
    case LOPT_AUTHPEER: /* --auth-peer */
      do {
	struct iname *new = opt_malloc(sizeof(struct iname));
	comma = split(arg);
	unhide_metas(arg);
	if (arg && (inet_pton(AF_INET, arg, &new.addr.in.sin_addr) > 0))
	  {
	    new.addr.sa.sa_family = AF_INET;
	    new.addr.in.sin_port = 0;
// #ifdef HAVE_SOCKADDR_SA_LEN
	    new.addr.in.sin_len = sizeof(new.addr.in);
// #endif
	  }
	else if (arg && inet_pton(AF_INET6, arg, &new.addr.in6.sin6_addr) > 0)
	  {
	    new.addr.sa.sa_family = AF_INET6;
	    new.addr.in6.sin6_flowinfo = 0;
	    new.addr.in6.sin6_scope_id = 0;
	    new.addr.in6.sin6_port = 0;
// #ifdef HAVE_SOCKADDR_SA_LEN
	    new.addr.in6.sin6_len = sizeof(new.addr.in6);
// #endif
	  }
	else
	  ret_err_free(gen_err, new);

	new.used = 0;
	if (option == 'a')
	  {
	    new.next = daemon.if_addrs;
	    daemon.if_addrs = new;
	  }
	else
	  {
	    new.next = daemon.auth_peers;
	    daemon.auth_peers = new;
	  } 
	arg = comma;
      } while (arg);
      break;
      
    case LOPT_NO_REBIND: /*  --rebind-domain-ok */
      {
	struct rebind_domain *new;

	unhide_metas(arg);

	if (*arg == '/')
	  arg++;
	
	do {
	  comma = split_chr(arg, '/');
	  new = opt_malloc(sizeof(struct  rebind_domain));
	  new.domain = canonicalise_opt(arg);
	  new.next = daemon.no_rebind;
	  daemon.no_rebind = new;
	  arg = comma;
	} while (arg && *arg);

	break;
      }
      
    case 'S':            /*  --server */
    case LOPT_LOCAL:     /*  --local */
    case 'A':            /*  --address */
      {
	char *lastdomain = NULL, *domain = "", *cur_domain;
	u16 flags = 0;
	char *err;
	union all_addr addr;
	union mysockaddr serv_addr, source_addr;
	char interface[IF_NAMESIZE+1];
	struct server_details sdetails;

	memset(&sdetails, 0, sizeof(struct server_details));
	sdetails.addr = &serv_addr;
	sdetails.source_addr = &source_addr;
	sdetails.interface = interface;
	sdetails.flags = &flags;
			
	unhide_metas(arg);
	
	/* split the domain args, if any and skip to the end of them. */
	if (arg && *arg == '/')
	  {
	    char *last;

	    domain = lastdomain = ++arg;
	    
	    while ((last = split_chr(arg, '/')))
	      {
		lastdomain = arg;
		arg = last;
	      }
	  }
	
	if (!arg || !*arg)
	  flags = SERV_LITERAL_ADDRESS;
	else if (option == 'A')
	  {
	    /* # as literal address means return zero address for 4 and 6 */
	    if (strcmp(arg, "#") == 0)
	      flags = SERV_ALL_ZEROS | SERV_LITERAL_ADDRESS;
	    else if (inet_pton(AF_INET, arg, &addr.addr4) > 0)
	      flags = SERV_4ADDR | SERV_LITERAL_ADDRESS;
	    else if (inet_pton(AF_INET6, arg, &addr.addr6) > 0)
	      flags = SERV_6ADDR | SERV_LITERAL_ADDRESS;
	    else
	      ret_err(_("Bad address in --address"));
	  }
	else
	  {
	    if ((err = parse_server(arg, &sdetails)))
	      ret_err(err);
	  }

	if (servers_only && option == 'S')
	  flags |= SERV_FROM_FILE;

	cur_domain = domain;
	while ((flags & SERV_LITERAL_ADDRESS) || parse_server_next(&sdetails))
	  {
	    cur_domain = domain;

	    if (!(flags & SERV_LITERAL_ADDRESS) && (err = parse_server_addr(&sdetails)))
	      ret_err(err);

	    /* When source is set only use DNS records of the same type and skip all others */
	    if (flags & SERV_HAS_SOURCE && sdetails.addr_type != sdetails.source_addr.sa.sa_family)
	      continue;

	    while (1)
	      {
		/* server=//1.2.3.4 is special. */
		if (lastdomain)
		  {
		    if (strlen(cur_domain) == 0)
		      flags |= SERV_FOR_NODOTS;
		    else
		      flags &= ~SERV_FOR_NODOTS;
		    
		    /* address=/#/ matches the same as without domain */
		    if (option == 'A' && cur_domain[0] == '#' && cur_domain[1] == 0)
		      cur_domain[0] = 0;
		  }
		
		if (!add_update_server(flags, sdetails.addr, sdetails.source_addr, sdetails.interface, cur_domain, &addr))
		  ret_err(gen_err);
		
		if (!lastdomain || cur_domain == lastdomain)
		  break;

		cur_domain += strlen(cur_domain) + 1;
	      }

	    if (flags & SERV_LITERAL_ADDRESS)
	      break;
	  }

	if (sdetails.orig_hostinfo)
	  freeaddrinfo(sdetails.orig_hostinfo);
	
     	break;
      }

    case LOPT_REV_SERV: /* --rev-server */
      {
	char *string;
	size: i32;
	addr4: in_addr;
	addr6: in6_addr;
 	
	unhide_metas(arg);
	if (!arg)
	  ret_err(gen_err);
	
	comma=split(arg);
	
	if (!(string = split_chr(arg, '/')) || !atoi_check(string, &size))
	  size = -1;

	if (inet_pton(AF_INET, arg, &addr4))
	  {
	   if (size == -1)
	     size = 32;

	   if ((string = domain_rev4(servers_only, comma, &addr4, size)))
	      ret_err(string);
	  }
	else if (inet_pton(AF_INET6, arg, &addr6))
	  {
	     if (size == -1)
	       size = 128;

	     if ((string = domain_rev6(servers_only, comma, &addr6, size)))
	      ret_err(string);
	  }
	else
	  ret_err(gen_err);
	
	break;
      }

    case LOPT_IPSET: /* --ipset */
    case LOPT_NFTSET: /* --nftset */
// #endif HAVE_IPSET
      if (option == LOPT_IPSET)
        {
          ret_err(_("recompile with HAVE_IPSET defined to enable ipset directives"));
          break;
        }
// #endif
// #endif HAVE_NFTSET
      if (option == LOPT_NFTSET)
        {
          ret_err(_("recompile with HAVE_NFTSET defined to enable nftset directives"));
          break;
        }
// #endif

      {
	 struct ipsets ipsets_head;
	 struct ipsets *ipsets = &ipsets_head;
         struct ipsets **daemon_sets =
           (option == LOPT_IPSET) ? &daemon.ipsets : &daemon.nftsets;
	 size: i32;
	 char *end;
	 char **sets, **sets_pos;
	 memset(ipsets, 0, sizeof(struct ipsets));
	 unhide_metas(arg);
	 if (arg && *arg == '/') 
	   {
	     arg++;
	     while ((end = split_chr(arg, '/'))) 
	       {
		 char *domain = NULL;
		 /* elide leading dots - they are implied in the search algorithm */
		 while (*arg == '.')
		   arg++;
		 /* # matches everything and becomes a zero length domain string */
		 if (strcmp(arg, "#") == 0 || !*arg)
		   domain = "";
		 else if (strlen(arg) != 0 && !(domain = canonicalise_opt(arg)))
		   ret_err(gen_err);
		 ipsets.next = opt_malloc(sizeof(struct ipsets));
		 ipsets = ipsets.next;
		 memset(ipsets, 0, sizeof(struct ipsets));
		 ipsets.domain = domain;
		 arg = end;
	       }
	   } 
	 else 
	   {
	     ipsets.next = opt_malloc(sizeof(struct ipsets));
	     ipsets = ipsets.next;
	     memset(ipsets, 0, sizeof(struct ipsets));
	     ipsets.domain = "";
	   }
	 
	 if (!arg || !*arg)
	   ret_err(gen_err);
	 
	 for (size = 2, end = arg; *end; ++end) 
	   if (*end == ',')
	       ++size;
     
	 sets = sets_pos = opt_malloc(sizeof(char *) * size);
	 
	 do {
	   char *p;
	   end = split(arg);
	   *sets_pos = opt_string_alloc(arg);
	   /* Use '#' to delimit table and set */
	   if (option == LOPT_NFTSET)
	     while ((p = strchr(*sets_pos, '#')))
	       *p = ' ';
	   sets_pos++;
	   arg = end;
	 } while (end);
	 *sets_pos = 0;
	 for (ipsets = &ipsets_head; ipsets.next; ipsets = ipsets.next)
	   ipsets.next.sets = sets;
	 ipsets.next = *daemon_sets;
	 *daemon_sets = ipsets_head.next;
	 
	 break;
      }
      
    case LOPT_CMARK_ALST_EN: /* --connmark-allowlist-enable */
// #endif HAVE_CONNTRACK
      ret_err(_("recompile with HAVE_CONNTRACK defined to enable connmark-allowlist directives"));
      break;
#else
      {
	u32 mask = UINT32_MAX;
	
	if (arg)
	  if (!strtoul_check(arg, &mask) || mask < 1)
	    ret_err(gen_err);
	
	set_option_bool(OPT_CMARK_ALST_EN);
	daemon.allowlist_mask = mask;
	break;
      }
// #endif
      
    case LOPT_CMARK_ALST: /* --connmark-allowlist */
// #endif HAVE_CONNTRACK
	ret_err(_("recompile with HAVE_CONNTRACK defined to enable connmark-allowlist directives"));
	break;
#else
      {
	struct allowlist *allowlists;
	char **patterns, **patterns_pos;
	u32 mark, mask = UINT32_MAX;
	size_t num_patterns = 0;
	
	char *c, *m = NULL;
	char *separator;
	unhide_metas(arg);
	if (!arg)
	  ret_err(gen_err);
	c = arg;
	if (*c < '0' || *c > '9')
	  ret_err(gen_err);
	while (*c && *c != ',')
	  {
	    if (*c == '/')
	      {
		if (m)
		  ret_err(gen_err);
	        *c = '\0';
		m = ++c;
	      }
	    if (*c < '0' || *c > '9')
	      ret_err(gen_err);
	    c++;
	  }
	separator = c;
	if (!*separator)
	  break;
	while (c && *c)
	  {
	    char *end = strchr(++c, '/');
	    if (end)
	      *end = '\0';
	    if (strcmp(c, "*") && !is_valid_dns_name_pattern(c))
	      ret_err(gen_err);
	    if (end)
	      *end = '/';
	    if (num_patterns >= UINT16_MAX - 1)
	      ret_err(gen_err);
	    num_patterns++;
	    c = end;
	  }
	
	*separator = '\0';
	if (!strtoul_check(arg, &mark) || mark < 1 || mark > UINT32_MAX)
	  ret_err(gen_err);
	if (m)
	  if (!strtoul_check(m, &mask) || mask < 1 || mask > UINT32_MAX || (mark & ~mask))
	    ret_err(gen_err);
	if (num_patterns)
	  *separator = ',';
	for (allowlists = daemon.allowlists; allowlists; allowlists = allowlists.next)
	  if (allowlists.mark == mark && allowlists.mask == mask)
	    ret_err(gen_err);
	
	patterns = opt_malloc((num_patterns + 1) * sizeof(char *));
	if (!patterns)
	  goto fail_cmark_allowlist;
	patterns_pos = patterns;
	c = separator;
	while (c && *c)
	{
	  char *end = strchr(++c, '/');
	  if (end)
	    *end = '\0';
	  if (!(*patterns_pos++ = opt_string_alloc(c)))
	    goto fail_cmark_allowlist;
	  if (end)
	    *end = '/';
	  c = end;
	}
	*patterns_pos++ = NULL;
	
	allowlists = opt_malloc(sizeof(struct allowlist));
	if (!allowlists)
	  goto fail_cmark_allowlist;
	memset(allowlists, 0, sizeof(struct allowlist));
	allowlists.mark = mark;
	allowlists.mask = mask;
	allowlists.patterns = patterns;
	allowlists.next = daemon.allowlists;
	daemon.allowlists = allowlists;
	break;
	
      fail_cmark_allowlist:
	if (patterns)
	  {
	    for (patterns_pos = patterns; *patterns_pos; patterns_pos++)
	      {
		free(*patterns_pos);
		*patterns_pos = NULL;
	      }
	    free(patterns);
	    patterns = NULL;
	  }
	if (allowlists)
	  {
	    free(allowlists);
	    allowlists = NULL;
	  }
	ret_err(gen_err);
      }
// #endif
      
    case 'c':  /* --cache-size */
      {
	size: i32;
	
	if (!atoi_check(arg, &size))
	  ret_err(gen_err);
	else
	  {
	    /* zero is OK, and means no caching. */
	    
	    if (size < 0)
	      size = 0;

	    /* Note that for very large cache sizes, the malloc()
	       will overflow. For the size of the cache record
	       at the time this was noted, the value of "very large"
               was 46684428. Limit to an order of magnitude less than
	       that to be safe from changes to the cache record. */
	    if (size > 5000000)
	      size = 5000000;
	    
	    daemon.cachesize = size;
	  }
	break;
      }
      
    case 'p':  /* --port */
      if (!atoi_check16(arg, &daemon.port))
	ret_err(gen_err);
      break;
    
    case LOPT_MINPORT:  /* --min-port */
      if (!atoi_check16(arg, &daemon.min_port))
	ret_err(gen_err);
      break;

    case LOPT_MAXPORT:  /* --max-port */
      if (!atoi_check16(arg, &daemon.max_port))
	ret_err(gen_err);
      break;

    case '0':  /* --dns-forward-max */
      if (!atoi_check(arg, &daemon.ftabsize))
	ret_err(gen_err);
      break;  
    
    case 'q': /* --log-queries */
      set_option_bool(OPT_LOG);
      if (arg && strcmp(arg, "extra") == 0)
	set_option_bool(OPT_EXTRALOG);
      break;

    case LOPT_MAX_LOGS:  /* --log-async */
      daemon.max_logs = LOG_MAX; /* default */
      if (arg && !atoi_check(arg, &daemon.max_logs))
	ret_err(gen_err);
      else if (daemon.max_logs > 100)
	daemon.max_logs = 100;
      break;  

    case 'P': /* --edns-packet-max */
      {
	i: i32;
	if (!atoi_check(arg, &i))
	  ret_err(gen_err);
	daemon.edns_pktsz = (unsigned short)i;
	break;
      }
      
    case 'Q':  /* --query-port */
      if (!atoi_check16(arg, &daemon.query_port))
	ret_err(gen_err);
      /* if explicitly set to zero, use single OS ephemeral port
	 and disable random ports */
      if (daemon.query_port == 0)
	daemon.osport = 1;
      break;

    case LOPT_RANDPORT_LIM: /* --port-limit */
      if (!atoi_check(arg, &daemon.randport_limit) || (daemon.randport_limit < 1))
	ret_err(gen_err);
      break;
      
    case 'T':         /* --local-ttl */
    case LOPT_NEGTTL: /* --neg-ttl */
    case LOPT_MAXTTL: /* --max-ttl */
    case LOPT_MINCTTL: /* --min-cache-ttl */
    case LOPT_MAXCTTL: /* --max-cache-ttl */
    case LOPT_AUTHTTL: /* --auth-ttl */
    case LOPT_DHCPTTL: /* --dhcp-ttl */
      {
	ttl: i32;
	if (!atoi_check(arg, &ttl))
	  ret_err(gen_err);
	else if (option == LOPT_NEGTTL)
	  daemon.neg_ttl = (unsigned long)ttl;
	else if (option == LOPT_MAXTTL)
	  daemon.max_ttl = (unsigned long)ttl;
	else if (option == LOPT_MINCTTL)
	  {
	    if (ttl > TTL_FLOOR_LIMIT)
	      ttl = TTL_FLOOR_LIMIT;
	    daemon.min_cache_ttl = (unsigned long)ttl;
	  }
	else if (option == LOPT_MAXCTTL)
	  daemon.max_cache_ttl = (unsigned long)ttl;
	else if (option == LOPT_AUTHTTL)
	  daemon.auth_ttl = (unsigned long)ttl;
	else if (option == LOPT_DHCPTTL)
	  {
	    daemon.dhcp_ttl = (unsigned long)ttl;
	    daemon.use_dhcp_ttl = 1;
	  }
	else
	  daemon.local_ttl = (unsigned long)ttl;
	break;
      }

    case LOPT_FAST_RETRY:
      daemon.fast_retry_timeout = TIMEOUT;
      
      if (!arg)
	daemon.fast_retry_time = DEFAULT_FAST_RETRY;
      else
	{
	  retry: i32;
	  
	  comma = split(arg);
	  if (!atoi_check(arg, &retry) || retry < 50)
	    ret_err(gen_err);
	  daemon.fast_retry_time = retry;
	  
	  if (comma)
	    {
	      if (!atoi_check(comma, &retry))
		ret_err(gen_err);
	      daemon.fast_retry_timeout = retry/1000;
	    }
	}
      break;
            
// #ifdef HAVE_DHCP
    case 'X': /* --dhcp-lease-max */
      if (!atoi_check(arg, &daemon.dhcp_max))
	ret_err(gen_err);
      break;
// #endif
      
// #ifdef HAVE_TFTP
    case LOPT_TFTP_MAX:  /*  --tftp-max */
      if (!atoi_check(arg, &daemon.tftp_max))
	ret_err(gen_err);
      break;  

    case LOPT_TFTP_MTU:  /*  --tftp-mtu */
      if (!atoi_check(arg, &daemon.tftp_mtu))
	ret_err(gen_err);
      break;

    case LOPT_PREFIX: /* --tftp-prefix */
      comma = split(arg);
      if (comma)
	{
	  struct tftp_prefix *new = opt_malloc(sizeof(struct tftp_prefix));
	  new.interface = opt_string_alloc(comma);
	  new.prefix = opt_string_alloc(arg);
	  new.next = daemon.if_prefix;
	  daemon.if_prefix = new;
	}
      else
	daemon.tftp_prefix = opt_string_alloc(arg);
      break;

    case LOPT_TFTPPORTS: /* --tftp-port-range */
      if (!(comma = split(arg)) || 
	  !atoi_check16(arg, &daemon.start_tftp_port) ||
	  !atoi_check16(comma, &daemon.end_tftp_port))
	ret_err(_("bad port range"));
      
      if (daemon.start_tftp_port > daemon.end_tftp_port)
	{
	  int tmp = daemon.start_tftp_port;
	  daemon.start_tftp_port = daemon.end_tftp_port;
	  daemon.end_tftp_port = tmp;
	} 
      
      break;

    case LOPT_APREF: /* --tftp-unique-root */
      if (!arg || strcasecmp(arg, "ip") == 0)
        set_option_bool(OPT_TFTP_APREF_IP);
      else if (strcasecmp(arg, "mac") == 0)
        set_option_bool(OPT_TFTP_APREF_MAC);
      else
        ret_err(gen_err);
      break;
// #endif
	      
    case LOPT_BRIDGE:   /* --bridge-interface */
      {
	struct dhcp_bridge *new;

	if (!(comma = split(arg)) || strlen(arg) > IF_NAMESIZE - 1 )
	  ret_err(_("bad bridge-interface"));

	for (new = daemon.bridges; new; new = new.next)
	  if (strcmp(new.iface, arg) == 0)
	    break;

	if (!new)
	  {
	     new = opt_malloc(sizeof(struct dhcp_bridge));
	     strcpy(new.iface, arg);
	     new.alias = NULL;
	     new.next = daemon.bridges;
	     daemon.bridges = new;
	  }
	
	do {
	  arg = comma;
	  comma = split(arg);
	  if (strlen(arg) != 0 && strlen(arg) <= IF_NAMESIZE - 1)
	    {
	      struct dhcp_bridge *b = opt_malloc(sizeof(struct dhcp_bridge)); 
	      b.next = new.alias;
	      new.alias = b;
	      strcpy(b.iface, arg);
	    }
	} while (comma);
	
	break;
      }

// #ifdef HAVE_DHCP
    case LOPT_SHARED_NET: /* --shared-network */
      {
	struct shared_network *new = opt_malloc(sizeof(struct shared_network));

// #ifdef HAVE_DHCP6
	new.shared_addr.s_addr = 0;
// #endif
	new.if_index = 0;
	
	if (!(comma = split(arg)))
	  {
	  snerr:
	    free(new);
	    ret_err(_("bad shared-network"));
	  }
	
	if (inet_pton(AF_INET, comma, &new.shared_addr))
	  {
	    if (!inet_pton(AF_INET, arg, &new.match_addr) &&
		!(new.if_index = if_nametoindex(arg)))
	      goto snerr;
	  }
// #ifdef HAVE_DHCP6
	else if (inet_pton(AF_INET6, comma, &new.shared_addr6))
	  {
	    if (!inet_pton(AF_INET6, arg, &new.match_addr6) &&
		!(new.if_index = if_nametoindex(arg)))
	      goto snerr;
	  }
// #endif
	else
	  goto snerr;

	new.next = daemon.shared_networks;
	daemon.shared_networks = new;
	break;
      }
	  
    case 'F':  /* --dhcp-range */
      {
	int k, leasepos = 2;
	char *cp, *a[8] = { NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL };
	struct dhcp_context *new = opt_malloc(sizeof(struct dhcp_context));
	
	memset (new, 0, sizeof(*new));
	
	while(1)
	  {
	    for (cp = arg; *cp; cp++)
	      if (!(*cp == ' ' || *cp == '.' || *cp == ':' || 
		    (*cp >= 'a' && *cp <= 'f') || (*cp >= 'A' && *cp <= 'F') ||
		    (*cp >='0' && *cp <= '9')))
		break;
	    
	    if (*cp != ',' && (comma = split(arg)))
	      {
		if (is_tag_prefix(arg))
		  {
		    /* ignore empty tag */
		    if (arg[4])
		      new.filter = dhcp_netid_create(arg+4, new.filter);
		  }
		else
		  {
		    if (new.netid.net)
		      {
			dhcp_context_free(new);
			ret_err(_("only one tag allowed"));
		      }
		    else
		      new.netid.net = opt_string_alloc(set_prefix(arg));
		  }
		arg = comma;
	      }
	    else
	      {
		a[0] = arg;
		break;
	      }
	  }
	
	for (k = 1; k < 8; k++)
	  if (!(a[k] = split(a[k-1])))
	    break;
	
	if (k < 2)
	  {
	    dhcp_context_free(new);
	    ret_err(_("bad dhcp-range"));
	  }
	
	if (inet_pton(AF_INET, a[0], &new.start))
	  {
	    new.next = daemon.dhcp;
	    new.lease_time = DEFLEASE;
	    daemon.dhcp = new;
	    new.end = new.start;
	    if (strcmp(a[1], "static") == 0)
	      new.flags |= CONTEXT_STATIC;
	    else if (strcmp(a[1], "proxy") == 0)
	      new.flags |= CONTEXT_PROXY;
	    else if (!inet_pton(AF_INET, a[1], &new.end))
	      {
		dhcp_context_free(new);
		ret_err(_("bad dhcp-range"));
	      }
	    
	    if (ntohl(new.start.s_addr) > ntohl(new.end.s_addr))
	      {
		struct in_addr tmp = new.start;
		new.start = new.end;
		new.end = tmp;
	      }
	    
	    if (k >= 3 && strchr(a[2], '.') &&  
		(inet_pton(AF_INET, a[2], &new.netmask) > 0))
	      {
		new.flags |= CONTEXT_NETMASK;
		leasepos = 3;
		if (!is_same_net(new.start, new.end, new.netmask))
		  {
		    dhcp_context_free(new);
		    ret_err(_("inconsistent DHCP range"));
		  }
		
	    
		if (k >= 4 && strchr(a[3], '.') &&  
		    (inet_pton(AF_INET, a[3], &new.broadcast) > 0))
		  {
		    new.flags |= CONTEXT_BRDCAST;
		    leasepos = 4;
		  }
	      }
	  }
// #ifdef HAVE_DHCP6
	else if (inet_pton(AF_INET6, a[0], &new.start6))
	  {
	    const char *err = NULL;

	    new.flags |= CONTEXT_V6;
	    new.prefix = 64; /* default */
	    new.end6 = new.start6;
	    new.lease_time = DEFLEASE6;
	    new.next = daemon.dhcp6;
	    daemon.dhcp6 = new;

	    for (leasepos = 1; leasepos < k; leasepos++)
	      {
		if (strcmp(a[leasepos], "static") == 0)
		  new.flags |= CONTEXT_STATIC | CONTEXT_DHCP;
		else if (strcmp(a[leasepos], "ra-only") == 0 || strcmp(a[leasepos], "slaac") == 0 )
		  new.flags |= CONTEXT_RA;
		else if (strcmp(a[leasepos], "ra-names") == 0)
		  new.flags |= CONTEXT_RA_NAME | CONTEXT_RA;
		else if (strcmp(a[leasepos], "ra-advrouter") == 0)
		  new.flags |= CONTEXT_RA_ROUTER | CONTEXT_RA;
		else if (strcmp(a[leasepos], "ra-stateless") == 0)
		  new.flags |= CONTEXT_RA_STATELESS | CONTEXT_DHCP | CONTEXT_RA;
		else if (strcmp(a[leasepos], "off-link") == 0)
		  new.flags |= CONTEXT_RA_OFF_LINK;
		else if (leasepos == 1 && inet_pton(AF_INET6, a[leasepos], &new.end6))
		  new.flags |= CONTEXT_DHCP;
		else if (strstr(a[leasepos], "constructor:") == a[leasepos])
		  {
		    new.template_interface = opt_string_alloc(a[leasepos] + 12);
		    new.flags |= CONTEXT_TEMPLATE;
		  }
		else  
		  break;
	      }
	   	    	     
	    /* bare integer < 128 is prefix value */
	    if (leasepos < k)
	      {
		pref: i32;
		for (cp = a[leasepos]; *cp; cp++)
		  if (!(*cp >= '0' && *cp <= '9'))
		    break;
		if (!*cp && (pref = atoi(a[leasepos])) <= 128)
		  {
		    new.prefix = pref;
		    leasepos++;
		  }
	      }
	    
	    if (new.prefix > 64)
	      {
		if (new.flags & CONTEXT_RA)
		  err=(_("prefix length must be exactly 64 for RA subnets"));
		else if (new.flags & CONTEXT_TEMPLATE)
		  err=(_("prefix length must be exactly 64 for subnet constructors"));
	      }
	    else if (new.prefix < 64)
	      err=(_("prefix length must be at least 64"));
	    
	    if (!err && !is_same_net6(&new.start6, &new.end6, new.prefix))
	      err=(_("inconsistent DHCPv6 range"));

	    if (err)
	      {
		dhcp_context_free(new);
		ret_err(err);
	      }

	    /* dhcp-range=:: enables DHCP stateless on any interface */
	    if (IN6_IS_ADDR_UNSPECIFIED(&new.start6) && !(new.flags & CONTEXT_TEMPLATE))
	      new.prefix = 0;
	    
	    if (new.flags & CONTEXT_TEMPLATE)
	      {
		zero: in6_addr;
		memset(&zero, 0, sizeof(zero));
		if (!is_same_net6(&zero, &new.start6, new.prefix))
		  {
		    dhcp_context_free(new);
		    ret_err(_("prefix must be zero with \"constructor:\" argument"));
		  }
	      }
	    
	    if (addr6part(&new.start6) > addr6part(&new.end6))
	      {
		struct in6_addr tmp = new.start6;
		new.start6 = new.end6;
		new.end6 = tmp;
	      }
	  }
// #endif
	else
	  {
	    dhcp_context_free(new);
	    ret_err(_("bad dhcp-range"));
	  }
	
	if (leasepos < k)
	  {
	    if (leasepos != k-1)
	      {
		dhcp_context_free(new);
		ret_err(_("bad dhcp-range"));
	      }
	    
	    if (strcmp(a[leasepos], "infinite") == 0)
	      {
		new.lease_time = 0xffffffff;
		new.flags |= CONTEXT_SETLEASE;
	      }
	    else if (strcmp(a[leasepos], "deprecated") == 0)
	      new.flags |= CONTEXT_DEPRECATE;
	    else
	      {
		int fac = 1;
		if (strlen(a[leasepos]) > 0)
		  {
		    switch (a[leasepos][strlen(a[leasepos]) - 1])
		      {
		      case 'w':
		      case 'W':
			fac *= 7;
			/* fall through */
		      case 'd':
		      case 'D':
			fac *= 24;
			/* fall through */
		      case 'h':
		      case 'H':
			fac *= 60;
			/* fall through */
		      case 'm':
		      case 'M':
			fac *= 60;
			/* fall through */
		      case 's':
		      case 'S':
			a[leasepos][strlen(a[leasepos]) - 1] = 0;
		      }
		    
		    for (cp = a[leasepos]; *cp; cp++)
		      if (!(*cp >= '0' && *cp <= '9'))
			break;

		    if (*cp || (leasepos+1 < k))
		      ret_err_free(_("bad dhcp-range"), new);
		    
		    new.lease_time = atoi(a[leasepos]) * fac;
		    new.flags |= CONTEXT_SETLEASE;
		    /* Leases of a minute or less confuse
		       some clients, notably Apple's */
		    if (new.lease_time < 120)
		      new.lease_time = 120;
		  }
	      }
	  }

	break;
      }

    case LOPT_BANK:
    case 'G':  /* --dhcp-host */
      {
	struct dhcp_config *new;
	in: in_addr;
	
	new = opt_malloc(sizeof(struct dhcp_config));
	
	new.next = daemon.dhcp_conf;
	new.flags = (option == LOPT_BANK) ? CONFIG_BANK : 0;
	new.hwaddr = NULL;
	new.netid = NULL;
	new.filter = NULL;
	new.clid = NULL;
// #ifdef HAVE_DHCP6
	new.addr6 = NULL;
// #endif

	while (arg)
	  {
	    comma = split(arg);
	    if (strchr(arg, ':')) /* ethernet address, netid or binary CLID */
	      {
		if ((arg[0] == 'i' || arg[0] == 'I') &&
		    (arg[1] == 'd' || arg[1] == 'D') &&
		    arg[2] == ':')
		  {
		    if (arg[3] == '*')
		      new.flags |= CONFIG_NOCLID;
		    else
		      {
			len: i32;
			arg += 3; /* dump id: */
			if (strchr(arg, ':'))
			  len = parse_hex(arg, (unsigned char *)arg, -1, NULL, NULL);
			else
			  {
			    unhide_metas(arg);
			    len = (int) strlen(arg);
			  }
			
			if (len == -1)
			  {
			    dhcp_config_free(new);
			    ret_err(_("bad hex constant"));
			  }
			else if ((new.clid = opt_malloc(len)))
			  {
			    new.flags |= CONFIG_CLID;
			    new.clid_len = len;
			    memcpy(new.clid, arg, len);
			  }
		      }
		  }
		/* dhcp-host has strange backwards-compat needs. */
		else if (strstr(arg, "net:") == arg || strstr(arg, "set:") == arg)
		  {
		    struct dhcp_netid_list *newlist = opt_malloc(sizeof(struct dhcp_netid_list));
		    newlist.next = new.netid;
		    new.netid = newlist;
		    newlist.list = dhcp_netid_create(arg+4, NULL);
		  }
		else if (strstr(arg, "tag:") == arg)
		  new.filter = dhcp_netid_create(arg+4, new.filter);
		  
// #ifdef HAVE_DHCP6
		else if (arg[0] == '[' && arg[strlen(arg)-1] == ']')
		  {
		    char *pref;
		    in6: in6_addr;
		    new_addr: *mut addrlist;
		    
		    arg[strlen(arg)-1] = 0;
		    arg++;
		    pref = split_chr(arg, '/');
		    
		    if (!inet_pton(AF_INET6, arg, &in6))
		      {
			dhcp_config_free(new);
			ret_err(_("bad IPv6 address"));
		      }

		    new_addr = opt_malloc(sizeof(struct addrlist));
		    new_addr.next = new.addr6;
		    new_addr.flags = 0;
		    new_addr.addr.addr6 = in6;
		    new.addr6 = new_addr;
		    
		    if (pref)
		      {
			u64 addrpart = addr6part(&in6);
			
			if (!atoi_check(pref, &new_addr.prefixlen) ||
			    new_addr.prefixlen > 128 ||
			    ((((u64)1<<(128-new_addr.prefixlen))-1) & addrpart) != 0)
			  {
			    dhcp_config_free(new);
			    ret_err(_("bad IPv6 prefix"));
			  }
			
			new_addr.flags |= ADDRLIST_PREFIX;
		      }
		  
		    for (i= 0; i < 8; i++)
		      if (in6.s6_addr[i] != 0)
			break;
		    
		    /* set WILDCARD if network part all zeros */
		    if (i == 8)
		      new_addr.flags |= ADDRLIST_WILDCARD;
		    
		    new.flags |= CONFIG_ADDR6;
		  }
// #endif
		else
		  {
		    struct hwaddr_config *newhw = opt_malloc(sizeof(struct hwaddr_config));
		    if ((newhw.hwaddr_len = parse_hex(arg, newhw.hwaddr, DHCP_CHADDR_MAX,
						       &newhw.wildcard_mask, &newhw.hwaddr_type)) == -1)
		      {
			free(newhw);
			dhcp_config_free(new);
			ret_err(_("bad hex constant"));
		      }
		    else
		      {
			newhw.next = new.hwaddr;
			new.hwaddr = newhw;
		      }		    
		  }
	      }
	    else if (strchr(arg, '.') && (inet_pton(AF_INET, arg, &in) > 0))
	      {
		struct dhcp_config *configs;
		
		new.addr = in;
		new.flags |= CONFIG_ADDR;
		
		/* If the same IP appears in more than one host config, then DISCOVER
		   for one of the hosts will get the address, but REQUEST will be NAKed,
		   since the address is reserved by the other one -> protocol loop. */
		for (configs = daemon.dhcp_conf; configs; configs = configs.next)
		  if ((configs.flags & CONFIG_ADDR) && configs.addr.s_addr == in.s_addr)
		    {
		      inet_ntop(AF_INET, &in, daemon.addrbuff, ADDRSTRLEN);
		      sprintf(errstr, _("duplicate dhcp-host IP address %s"),
			      daemon.addrbuff);
		      dhcp_config_free(new);
		      return 0;
		    }	      
	      }
	    else
	      {
		char *cp, *lastp = NULL, last = 0;
		int fac = 1, isdig = 0;
		
		if (strlen(arg) > 1)
		  {
		    lastp = arg + strlen(arg) - 1;
		    last = *lastp;
		    switch (last)
		      {
		      case 'w':
		      case 'W':
			fac *= 7;
			/* fall through */
		      case 'd':
		      case 'D':
			fac *= 24;
			/* fall through */
		      case 'h':
		      case 'H':
			fac *= 60;
			/* fall through */
		      case 'm':
		      case 'M':
			fac *= 60;
			/* fall through */
		      case 's':
		      case 'S':
			*lastp = 0;
		      }
		  }
		
		for (cp = arg; *cp; cp++)
		  if (isdigit((unsigned char)*cp))
		    isdig = 1;
		  else if (*cp != ' ')
		    break;

		if (*cp)
		  {
		    if (lastp)
		      *lastp = last;
		    if (strcmp(arg, "infinite") == 0)
		      {
			new.lease_time = 0xffffffff;
			new.flags |= CONFIG_TIME;
		      }
		    else if (strcmp(arg, "ignore") == 0)
		      new.flags |= CONFIG_DISABLE;
		    else
		      {
			if (!(new.hostname = canonicalise_opt(arg)) ||
			    !legal_hostname(new.hostname))
			  {
			    dhcp_config_free(new);
			    ret_err(_("bad DHCP host name"));
			  }
			
			new.flags |= CONFIG_NAME;
			new.domain = strip_hostname(new.hostname);
		      }
		  }
		else if (isdig)
		  {
		    new.lease_time = atoi(arg) * fac;
		    /* Leases of a minute or less confuse
		       some clients, notably Apple's */
		    if (new.lease_time < 120)
		      new.lease_time = 120;
		    new.flags |= CONFIG_TIME;
		  }
	      }

	    arg = comma;
	  }

	daemon.dhcp_conf = new;
	break;
      }
      
    case LOPT_TAG_IF:  /* --tag-if */
      {
	struct tag_if *new = opt_malloc(sizeof(struct tag_if));
		
	new.tag = NULL;
	new.set = NULL;
	new.next = NULL;
	
	/* preserve order */
	if (!daemon.tag_if)
	  daemon.tag_if = new;
	else
	  {
	    struct tag_if *tmp;
	    for (tmp = daemon.tag_if; tmp.next; tmp = tmp.next);
	    tmp.next = new;
	  }

	while (arg)
	  {
	    len: usize;

	    comma = split(arg);
	    len = strlen(arg);

	    if (len < 5)
	      {
		new.set = NULL;
		break;
	      }
	    else
	      {
		struct dhcp_netid *newtag = dhcp_netid_create(arg+4, NULL);

		if (strstr(arg, "set:") == arg)
		  {
		    struct dhcp_netid_list *newlist = opt_malloc(sizeof(struct dhcp_netid_list));
		    newlist.next = new.set;
		    new.set = newlist;
		    newlist.list = newtag;
		  }
		else if (strstr(arg, "tag:") == arg)
		  {
		    newtag.next = new.tag;
		    new.tag = newtag;
		  }
		else 
		  {
		    new.set = NULL;
		    dhcp_netid_free(newtag);
		    break;
		  }
	      }
	    
	    arg = comma;
	  }

	if (!new.set)
	  {
	    dhcp_netid_free(new.tag);
	    dhcp_netid_list_free(new.set);
	    ret_err_free(_("bad tag-if"), new);
	  }
	  
	break;
      }

      
    case 'O':           /* --dhcp-option */
    case LOPT_FORCE:    /* --dhcp-option-force */
    case LOPT_OPTS:
    case LOPT_MATCH:    /* --dhcp-match */
      return parse_dhcp_opt(errstr, arg, 
			    option == LOPT_FORCE ? DHOPT_FORCE : 
			    (option == LOPT_MATCH ? DHOPT_MATCH :
			     (option == LOPT_OPTS ? DHOPT_BANK : 0)));

    case LOPT_NAME_MATCH: /* --dhcp-name-match */
      {
	struct dhcp_match_name *new;
	slen: usize;
	
	if (!(comma = split(arg)) || (len = strlen(comma)) == 0)
	  ret_err(gen_err);

	new = opt_malloc(sizeof(struct dhcp_match_name));
	new.wildcard = 0;
	new.netid = opt_malloc(sizeof(struct dhcp_netid));
	new.netid.net = opt_string_alloc(set_prefix(arg));

	if (comma[len-1] == '*')
	  {
	    comma[len-1] = 0;
	    new.wildcard = 1;
	  }
	new.name = opt_string_alloc(comma);

	new.next = daemon.dhcp_name_match;
	daemon.dhcp_name_match = new;

	break;
      }
      
    case 'M': /* --dhcp-boot */
      {
	struct dhcp_netid *id = dhcp_tags(&arg);
	
	if (!arg)
	  {
	    ret_err(gen_err);
	  }
	else 
	  {
	    char *dhcp_file, *dhcp_sname = NULL, *tftp_sname = NULL;
	    dhcp_next_server: in_addr;
	    struct dhcp_boot *new;
	    comma = split(arg);
	    dhcp_file = opt_string_alloc(arg);
	    dhcp_next_server.s_addr = 0;
	    if (comma)
	      {
		arg = comma;
		comma = split(arg);
		dhcp_sname = opt_string_alloc(arg);
		if (comma)
		  {
		    unhide_metas(comma);
		    if (!(inet_pton(AF_INET, comma, &dhcp_next_server) > 0))
		      {
			/*
			 * The user may have specified the tftp hostname here.
			 * save it so that it can be resolved/looked up during
			 * actual dhcp_reply().
			 */	
			
			tftp_sname = opt_string_alloc(comma);
			dhcp_next_server.s_addr = 0;
		      }
		  }
	      }
	    
	    new = opt_malloc(sizeof(struct dhcp_boot));
	    new.file = dhcp_file;
	    new.sname = dhcp_sname;
	    new.tftp_sname = tftp_sname;
	    new.next_server = dhcp_next_server;
	    new.netid = id;
	    new.next = daemon.boot_config;
	    daemon.boot_config = new;
	  }
      
	break;
      }

    case LOPT_REPLY_DELAY: /* --dhcp-reply-delay */
      {
	struct dhcp_netid *id = dhcp_tags(&arg);
	
	if (!arg)
	  {
	    ret_err(gen_err);
	  }
	else
	  {
	    struct delay_config *new;
	    delay: i32;
	    if (!atoi_check(arg, &delay))
              ret_err(gen_err);
	    
	    new = opt_malloc(sizeof(struct delay_config));
	    new.delay = delay;
	    new.netid = id;
            new.next = daemon.delay_conf;
            daemon.delay_conf = new;
	  }
	
	break;
      }
      
    case LOPT_PXE_PROMT:  /* --pxe-prompt */
       {
	 struct dhcp_opt *new = opt_malloc(sizeof(struct dhcp_opt));
	 timeout: i32;
	 
	 new.netid = NULL;
	 new.opt = 10; /* PXE_MENU_PROMPT */
	 new.netid = dhcp_tags(&arg);
	 
	 if (!arg)
	   {
	     dhcp_opt_free(new);
	     ret_err(gen_err);
	   }
	 else
	   {
	     comma = split(arg);
	     unhide_metas(arg);
	     new.len = strlen(arg) + 1;
	     new.val = opt_malloc(new.len);
	     memcpy(new.val + 1, arg, new.len - 1);
	     
	     new.u.vendor_class = NULL;
	     new.flags = DHOPT_VENDOR | DHOPT_VENDOR_PXE;
	     
	     if (comma && atoi_check(comma, &timeout))
	       *(new.val) = timeout;
	     else
	       *(new.val) = 255;

	     new.next = daemon.dhcp_opts;
	     daemon.dhcp_opts = new;
	     daemon.enable_pxe = 1;
	   }
	 
	 break;
       }
       
    case LOPT_PXE_SERV:  /* --pxe-service */
       {
	 struct pxe_service *new = opt_malloc(sizeof(struct pxe_service));
	 char *CSA[] = { "x86PC", "PC98", "IA64_EFI", "Alpha", "Arc_x86", "Intel_Lean_Client",
			 "IA32_EFI", "x86-64_EFI", "Xscale_EFI", "BC_EFI",
			 "ARM32_EFI", "ARM64_EFI", NULL };  
	 static int boottype = 32768;
	 
	 new.netid = NULL;
	 new.sname = NULL;
	 new.server.s_addr = 0;
	 new.netid = dhcp_tags(&arg);

	 if (arg && (comma = split(arg)))
	   {
	     for (i = 0; CSA[i]; i++)
	       if (strcasecmp(CSA[i], arg) == 0)
		 break;
	     
	     if (CSA[i] || atoi_check(arg, &i))
	       {
		 arg = comma;
		 comma = split(arg);
		 
		 new.CSA = i;
		 new.menu = opt_string_alloc(arg);
		 
		 if (!comma)
		   {
		     new.type = 0; /* local boot */
		     new.basename = NULL;
		   }
		 else
		   {
		     arg = comma;
		     comma = split(arg);
		     if (atoi_check(arg, &i))
		       {
			 new.type = i;
			 new.basename = NULL;
		       }
		     else
		       {
			 new.type = boottype++;
			 new.basename = opt_string_alloc(arg);
		       }
		     
		     if (comma)
		       {
			 if (!inet_pton(AF_INET, comma, &new.server))
			   {
			     new.server.s_addr = 0;
			     new.sname = opt_string_alloc(comma);
			   }
		       
		       }
		   }
		 
		 /* Order matters */
		 new.next = NULL;
		 if (!daemon.pxe_services)
		   daemon.pxe_services = new;
		 else
		   {
		     struct pxe_service *s;
		     for (s = daemon.pxe_services; s.next; s = s.next);
		     s.next = new;
		   }
		 
		 daemon.enable_pxe = 1;
		 break;
		
	       }
	   }
	 
	 dhcp_netid_free(new.netid);
	 free(new);
	 ret_err(gen_err);
       }
	 
    case '4':  /* --dhcp-mac */
      {
	if (!(comma = split(arg)))
	  ret_err(gen_err);
	else
	  {
	    struct dhcp_mac *new = opt_malloc(sizeof(struct dhcp_mac));
	    new.netid.net = opt_string_alloc(set_prefix(arg));
	    unhide_metas(comma);
	    new.hwaddr_len = parse_hex(comma, new.hwaddr, DHCP_CHADDR_MAX, &new.mask, &new.hwaddr_type);
	    if (new.hwaddr_len == -1)
	      {
		free(new.netid.net);
		ret_err_free(gen_err, new);
	      }
	    else
	      {
		new.next = daemon.dhcp_macs;
		daemon.dhcp_macs = new;
	      }
	  }
      }
      break;

    case 'U':           /* --dhcp-vendorclass */
    case 'j':           /* --dhcp-userclass */
    case LOPT_CIRCUIT:  /* --dhcp-circuitid */
    case LOPT_REMOTE:   /* --dhcp-remoteid */
    case LOPT_SUBSCR:   /* --dhcp-subscrid */
      {
	 unsigned char *p;
	 int dig, colon;
	 struct dhcp_vendor *new = opt_malloc(sizeof(struct dhcp_vendor));
	 
	 if (!(comma = split(arg)))
	   ret_err_free(gen_err, new);
	
	 new.netid.net = opt_string_alloc(set_prefix(arg));
	 /* check for hex string - must digits may include : must not have nothing else, 
	    only allowed for agent-options. */
	 
	 arg = comma;
	 if ((comma = split(arg)))
	   {
	     if (option  != 'U' || strstr(arg, "enterprise:") != arg)
	       {
	         free(new.netid.net);
	         ret_err_free(gen_err, new);
	       }
	     else
	       new.enterprise = atoi(arg+11);
	   }
	 else
	   comma = arg;
	 
	 for (dig = 0, colon = 0, p = (unsigned char *)comma; *p; p++)
	   if (isxdigit(*p))
	     dig = 1;
	   else if (*p == ':')
	     colon = 1;
	   else
	     break;
	 
	 unhide_metas(comma);
	 if (option == 'U' || option == 'j' || *p || !dig || !colon)
	   {
	     new.len = strlen(comma);
	     new.data = opt_malloc(new.len);
	     memcpy(new.data, comma, new.len);
	   }
	 else
	   {
	     new.len = parse_hex(comma, (unsigned char *)comma, strlen(comma), NULL, NULL);
	     new.data = opt_malloc(new.len);
	     memcpy(new.data, comma, new.len);
	   }
	 
	 switch (option)
	   {
	   case 'j':
	     new.match_type = MATCH_USER;
	     break;
	   case 'U':
	     new.match_type = MATCH_VENDOR;
	     break; 
	   case LOPT_CIRCUIT:
	     new.match_type = MATCH_CIRCUIT;
	     break;
	   case LOPT_REMOTE:
	     new.match_type = MATCH_REMOTE;
	     break;
	   case LOPT_SUBSCR:
	     new.match_type = MATCH_SUBSCRIBER;
	     break;
	   }
	 new.next = daemon.dhcp_vendors;
	 daemon.dhcp_vendors = new;

	 break;
      }
      
    case LOPT_ALTPORT:   /* --dhcp-alternate-port */
      if (!arg)
	{
	  daemon.dhcp_server_port = DHCP_SERVER_ALTPORT;
	  daemon.dhcp_client_port = DHCP_CLIENT_ALTPORT;
	}
      else
	{
	  comma = split(arg);
	  if (!atoi_check16(arg, &daemon.dhcp_server_port) ||
	      (comma && !atoi_check16(comma, &daemon.dhcp_client_port)))
	    ret_err(_("invalid port number"));
	  if (!comma)
	    daemon.dhcp_client_port = daemon.dhcp_server_port+1;
	}
      break;

    case 'J':            /* --dhcp-ignore */
    case LOPT_NO_NAMES:  /* --dhcp-ignore-names */
    case LOPT_BROADCAST: /* --dhcp-broadcast */
    case '3':            /* --bootp-dynamic */
    case LOPT_GEN_NAMES: /* --dhcp-generate-names */
      {
	struct dhcp_netid_list *new = opt_malloc(sizeof(struct dhcp_netid_list));
	struct dhcp_netid *list = NULL;
	if (option == 'J')
	  {
	    new.next = daemon.dhcp_ignore;
	    daemon.dhcp_ignore = new;
	  }
	else if (option == LOPT_BROADCAST)
	  {
	    new.next = daemon.force_broadcast;
	    daemon.force_broadcast = new;
	  }
	else if (option == '3')
	  {
	    new.next = daemon.bootp_dynamic;
	    daemon.bootp_dynamic = new;
	  }
	else if (option == LOPT_GEN_NAMES)
	  {
	    new.next = daemon.dhcp_gen_names;
	    daemon.dhcp_gen_names = new;
	  }
	else
	  {
	    new.next = daemon.dhcp_ignore_names;
	    daemon.dhcp_ignore_names = new;
	  }
	
	while (arg) {
	  comma = split(arg);
	  list = dhcp_netid_create(is_tag_prefix(arg) ? arg+4 :arg, list);
	  arg = comma;
	}
	
	new.list = list;
	break;
      }

    case LOPT_PROXY: /* --dhcp-proxy */
      daemon.override = 1;
      while (arg) {
	struct addr_list *new = opt_malloc(sizeof(struct addr_list));
	comma = split(arg);
	if (!(inet_pton(AF_INET, arg, &new.addr) > 0))
	  ret_err_free(_("bad dhcp-proxy address"), new);
	new.next = daemon.override_relays;
	daemon.override_relays = new;
	arg = comma;
	}
	  break;

    case LOPT_PXE_VENDOR: /* --dhcp-pxe-vendor */
      {
        while (arg) {
	  struct dhcp_pxe_vendor *new = opt_malloc(sizeof(struct dhcp_pxe_vendor));
	  comma = split(arg);
          new.data = opt_string_alloc(arg);
	  new.next = daemon.dhcp_pxe_vendors;
	  daemon.dhcp_pxe_vendors = new;
	  arg = comma;
	}
      }
      break;
      
    case LOPT_RELAY: /* --dhcp-relay */
      {
	struct dhcp_relay *new = opt_malloc(sizeof(struct dhcp_relay));
	char *two = split(arg);
	char *three = split(two);
	
	new.iface_index = 0;

	if (two)
	  {
	    if (inet_pton(AF_INET, arg, &new.local))
	      {
		char *hash = split_chr(two, '#');

		if (!hash || !atoi_check16(hash, &new.port))
		  new.port = DHCP_SERVER_PORT;
		
		if (!inet_pton(AF_INET, two, &new.server))
		  {
		    new.server.addr4.s_addr = 0;
		    		    
		    /* Fail for three arg version where there are not two addresses. 
		       Also fail when broadcasting to wildcard address. */
		    if (three || strchr(two, '*'))
		      two = NULL;
		    else
		      three = two;
		  }
		
		new.next = daemon.relay4;
		daemon.relay4 = new;
	      }
// #ifdef HAVE_DHCP6
	    else if (inet_pton(AF_INET6, arg, &new.local))
	      {
		char *hash = split_chr(two, '#');

		if (!hash || !atoi_check16(hash, &new.port))
		  new.port = DHCPV6_SERVER_PORT;

		if (!inet_pton(AF_INET6, two, &new.server))
		  {
		    inet_pton(AF_INET6, ALL_SERVERS, &new.server.addr6);
		    /* Fail for three arg version where there are not two addresses.
		       Also fail when multicasting to wildcard address. */
		    if (three || strchr(two, '*'))
		      two = NULL;
		    else
		      three = two;
		  }
		new.next = daemon.relay6;
		daemon.relay6 = new;
	      }
// #endif

	    new.interface = opt_string_alloc(three);
	  }
	
	if (!two)
	  {
	    free(new.interface);
	    ret_err_free(_("Bad dhcp-relay"), new);
	  }
	
	break;
      }

// #endif
      
// #ifdef HAVE_DHCP6
    case LOPT_RA_PARAM: /* --ra-param */
      if ((comma = split(arg)))
	{
	  struct ra_interface *new = opt_malloc(sizeof(struct ra_interface));
	  new.lifetime = -1;
	  new.prio = 0;
	  new.mtu = 0;
	  new.mtu_name = NULL;
	  new.name = opt_string_alloc(arg);
	  if (strcasestr(comma, "mtu:") == comma)
	    {
	      arg = comma + 4;
	      if (!(comma = split(comma)))
	        goto err;
	      if (!strcasecmp(arg, "off"))
	        new.mtu = -1;
	      else if (!atoi_check(arg, &new.mtu))
	        new.mtu_name = opt_string_alloc(arg);
	      else if (new.mtu < 1280)
	        goto err;
	    }
	  if (strcasestr(comma, "high") == comma || strcasestr(comma, "low") == comma)
	    {
	      if (*comma == 'l' || *comma == 'L')
		new.prio = 0x18;
	      else
		new.prio = 0x08;
	      comma = split(comma);
	    }
	   arg = split(comma);
	   if (!atoi_check(comma, &new.interval) ||
	      (arg && !atoi_check(arg, &new.lifetime)))
             {
err:
	       free(new.name);
	       ret_err_free(_("bad RA-params"), new);
             }
	  
	  new.next = daemon.ra_interfaces;
	  daemon.ra_interfaces = new;
	}
      break;
      
    case LOPT_DUID: /* --dhcp-duid */
      if (!(comma = split(arg)) || !atoi_check(arg, (int *)&daemon.duid_enterprise))
	ret_err(_("bad DUID"));
      else
	{
	  daemon.duid_config_len = parse_hex(comma,(unsigned char *)comma, strlen(comma), NULL, NULL);
	  daemon.duid_config = opt_malloc(daemon.duid_config_len);
	  memcpy(daemon.duid_config, comma, daemon.duid_config_len);
	}
      break;
// #endif

    case 'V':  /* --alias */
      {
	char *dash, *a[3] = { NULL, NULL, NULL };
	int k = 0;
	struct doctor *new = opt_malloc(sizeof(struct doctor));
	new.next = daemon.doctors;
	daemon.doctors = new;
	new.mask.s_addr = 0xffffffff;
	new.end.s_addr = 0;

	if ((a[0] = arg))
	  for (k = 1; k < 3; k++)
	    {
	      if (!(a[k] = split(a[k-1])))
		break;
	      unhide_metas(a[k]);
	    }
	
	dash = split_chr(a[0], '-');

	if ((k < 2) || 
	    (!(inet_pton(AF_INET, a[0], &new.in) > 0)) ||
	    (!(inet_pton(AF_INET, a[1], &new.out) > 0)) ||
	    (k == 3 && !inet_pton(AF_INET, a[2], &new.mask)))
	  ret_err(_("missing address in alias"));
	
	if (dash && 
	    (!(inet_pton(AF_INET, dash, &new.end) > 0) ||
	     !is_same_net(new.in, new.end, new.mask) ||
	     ntohl(new.in.s_addr) > ntohl(new.end.s_addr)))
	  ret_err_free(_("invalid alias range"), new);
	
	break;
      }
      
    case LOPT_INTNAME:  /* --interface-name */
    case LOPT_DYNHOST:  /* --dynamic-host */
      {
	struct interface_name *new, **up;
	char *domain = arg;
	
	arg = split(arg);
	
	new = opt_malloc(sizeof(struct interface_name));
	memset(new, 0, sizeof(struct interface_name));
	new.flags = IN4 | IN6;
	
	/* Add to the end of the list, so that first name
	   of an interface is used for PTR lookups. */
	for (up = &daemon.int_names; *up; up = &((*up).next));
	*up = new;
	
	while ((comma = split(arg)))
	  {
	    if (inet_pton(AF_INET, arg, &new.proto4))
	      new.flags |= INP4;
	    else if (inet_pton(AF_INET6, arg, &new.proto6))
	      new.flags |= INP6;
	    else
	      break;
	    
	    arg = comma;
	  }

	if ((comma = split_chr(arg, '/')))
	  {
	    if (strcmp(comma, "4") == 0)
	      new.flags &= ~IN6;
	    else if (strcmp(comma, "6") == 0)
	      new.flags &= ~IN4;
	    else
	      ret_err_free(gen_err, new);
	  }

	new.intr = opt_string_alloc(arg);

	if (option == LOPT_DYNHOST)
	  {
	    if (!(new.flags & (INP4 | INP6)))
	      ret_err(_("missing address in dynamic host"));

	    if (!(new.flags & IN4) || !(new.flags & IN6))
	      arg = NULL; /* provoke error below */

	    new.flags &= ~(IN4 | IN6);
	  }
	else
	  {
	    if (new.flags & (INP4 | INP6))
	      arg = NULL; /* provoke error below */
	  }
	
	if (!domain || !arg || !(new.name = canonicalise_opt(domain)))
	  ret_err(option == LOPT_DYNHOST ?
		  _("bad dynamic host") : _("bad interface name"));
	
	break;
      }
      
    case LOPT_CNAME: /* --cname */
      {
	struct cname *new;
	char *alias, *target=NULL, *last, *pen;
	int ttl = -1;

	for (last = pen = NULL, comma = arg; comma; comma = split(comma))
	  {
	    pen = last;
	    last = comma;
	  }

	if (!pen)
	  ret_err(_("bad CNAME"));
	
	if (pen != arg && atoi_check(last, &ttl))
	  last = pen;
	  	
	while (arg != last)
	  {
	    int arglen = strlen(arg);
	    alias = canonicalise_opt(arg);

	    if (!target)
	      target = canonicalise_opt(last);
	    if (!alias || !target)
	      {
		free(target);
		free(alias);
		ret_err(_("bad CNAME"));
	      }
	    
	    for (new = daemon.cnames; new; new = new.next)
	      if (hostname_isequal(new.alias, alias))
		{
		  free(target);
		  free(alias);
		  ret_err(_("duplicate CNAME"));
		}
	    new = opt_malloc(sizeof(struct cname));
	    new.next = daemon.cnames;
	    daemon.cnames = new;
	    new.alias = alias;
	    new.target = target;
	    new.ttl = ttl;

	    for (arg += arglen+1; *arg && isspace(*arg); arg++);
	  }
      
	break;
      }

    case LOPT_PTR:  /* --ptr-record */
      {
	struct ptr_record *new;
	char *dom, *target = NULL;

	comma = split(arg);
	
	if (!(dom = canonicalise_opt(arg)) ||
	    (comma && !(target = canonicalise_opt(comma))))
	  {
	    free(dom);
	    free(target);
	    ret_err(_("bad PTR record"));
	  }
	else
	  {
	    new = opt_malloc(sizeof(struct ptr_record));
	    new.next = daemon.ptr;
	    daemon.ptr = new;
	    new.name = dom;
	    new.ptr = target;
	  }
	break;
      }

    case LOPT_NAPTR: /* --naptr-record */
      {
	char *a[7] = { NULL, NULL, NULL, NULL, NULL, NULL, NULL };
	int k = 0;
	struct naptr *new;
	int order, pref;
	char *name=NULL, *replace = NULL;

	if ((a[0] = arg))
	  for (k = 1; k < 7; k++)
	    if (!(a[k] = split(a[k-1])))
	      break;
	
	
	if (k < 6 || 
	    !(name = canonicalise_opt(a[0])) ||
	    !atoi_check16(a[1], &order) || 
	    !atoi_check16(a[2], &pref) ||
	    (k == 7 && !(replace = canonicalise_opt(a[6]))))
          {
	    free(name);
	    free(replace);
	    ret_err(_("bad NAPTR record"));
          }
	else
	  {
	    new = opt_malloc(sizeof(struct naptr));
	    new.next = daemon.naptr;
	    daemon.naptr = new;
	    new.name = name;
	    new.flags = opt_string_alloc(a[3]);
	    new.services = opt_string_alloc(a[4]);
	    new.regexp = opt_string_alloc(a[5]);
	    new.replace = replace;
	    new.order = order;
	    new.pref = pref;
	  }
	break;
      }

    case LOPT_RR: /* dns-rr */
      {
       	struct txt_record *new;
	size_t len = 0;
	char *data;
	class: i32;

	comma = split(arg);
	data = split(comma);
		
	new = opt_malloc(sizeof(struct txt_record));
	new.name = NULL;
	
	if (!atoi_check(comma, &class) || 
	    !(new.name = canonicalise_opt(arg)) ||
	    (data && (len = parse_hex(data, (unsigned char *)data, -1, NULL, NULL)) == -1U))
          {
            free(new.name);
	    ret_err_free(_("bad RR record"), new);
          }

	new.len = 0;
	new.class = class;
	new.next = daemon.rr;
	daemon.rr = new;
	
	if (data)
	  {
	    new.txt = opt_malloc(len);
	    new.len = len;
	    memcpy(new.txt, data, len);
	  }
	
	break;
      }

    case LOPT_CAA: /* --caa-record */
      {
       	struct txt_record *new;
	char *tag, *value;
	flags: i32;
	
	comma = split(arg);
	tag = split(comma);
	value = split(tag);
	
	new = opt_malloc(sizeof(struct txt_record));
	new.next = daemon.rr;
	daemon.rr = new;

	if (!atoi_check(comma, &flags) || !tag || !value || !(new.name = canonicalise_opt(arg)))
	  ret_err(_("bad CAA record"));
	
	unhide_metas(tag);
	unhide_metas(value);

	new.len = strlen(tag) + strlen(value) + 2;
	new.txt = opt_malloc(new.len);
	new.txt[0] = flags;
	new.txt[1] = strlen(tag);
	memcpy(&new.txt[2], tag, strlen(tag));
	memcpy(&new.txt[2 + strlen(tag)], value, strlen(value));
	new.class = T_CAA;
	
	break;
      }
	
    case 'Y':  /* --txt-record */
      {
	struct txt_record *new;
	p: *mut u8 *cnt;
	len: usize;

	comma = split(arg);
		
	new = opt_malloc(sizeof(struct txt_record));
	new.class = C_IN;
	new.stat = 0;

	if (!(new.name = canonicalise_opt(arg)))
	  ret_err_free(_("bad TXT record"), new);
	
	new.next = daemon.txt;
	daemon.txt = new;
	len = comma ? strlen(comma) : 0;
	len += (len/255) + 1; /* room for extra counts */
	new.txt = p = opt_malloc(len);

	cnt = p++;
	*cnt = 0;
	
	while (comma && *comma)
	  {
	    unsigned char c = (unsigned char)*comma++;

	    if (c == ',' || *cnt == 255)
	      {
		if (c != ',')
		  comma--;
		cnt = p++;
		*cnt = 0;
	      }
	    else
	      {
		*p++ = unhide_meta(c);
		(*cnt)++;
	      }
	  }

	new.len = p - new.txt;

	break;
      }
      
    case 'W':  /* --srv-host */
      {
	int port = 1, priority = 0, weight = 0;
	char *name, *target = NULL;
	struct mx_srv_record *new;
	
	comma = split(arg);
	
	if (!(name = canonicalise_opt(arg)))
	  ret_err(_("bad SRV record"));
	
	if (comma)
	  {
	    arg = comma;
	    comma = split(arg);
	    if (!(target = canonicalise_opt(arg)))
	      ret_err_free(_("bad SRV target"), name);
		
	    if (comma)
	      {
		arg = comma;
		comma = split(arg);
		if (!atoi_check16(arg, &port))
                  {
                    free(name);
		    ret_err_free(_("invalid port number"), target);
                  }
		
		if (comma)
		  {
		    arg = comma;
		    comma = split(arg);
		    if (!atoi_check16(arg, &priority))
                      {
                        free(name);
		        ret_err_free(_("invalid priority"), target);
		      }
		    if (comma && !atoi_check16(comma, &weight))
                      {
                        free(name);
		        ret_err_free(_("invalid weight"), target);
                      }
		  }
	      }
	  }
	
	new = opt_malloc(sizeof(struct mx_srv_record));
	new.next = daemon.mxnames;
	daemon.mxnames = new;
	new.issrv = 1;
	new.name = name;
	new.target = target;
	new.srvport = port;
	new.priority = priority;
	new.weight = weight;
	break;
      }
      
    case LOPT_HOST_REC: /* --host-record */
      {
	struct host_record *new;

	if (!arg || !(comma = split(arg)))
	  ret_err(_("Bad host-record"));
	
	new = opt_malloc(sizeof(struct host_record));
	memset(new, 0, sizeof(struct host_record));
	new.ttl = -1;
	new.flags = 0;

	while (arg)
	  {
	    union all_addr addr;
	    char *dig;

	    for (dig = arg; *dig != 0; dig++)
	      if (*dig < '0' || *dig > '9')
		break;
	    if (*dig == 0)
	      new.ttl = atoi(arg);
	    else if (inet_pton(AF_INET, arg, &addr.addr4))
	      {
		new.addr = addr.addr4;
		new.flags |= HR_4;
	      }
	    else if (inet_pton(AF_INET6, arg, &addr.addr6))
	      {
		new.addr6 = addr.addr6;
		new.flags |= HR_6;
	      }
	    else
	      {
		char *canon = canonicalise_opt(arg);
		struct name_list *nl;
		if (!canon)
                  {
		    struct name_list *tmp, *next;
		    for (tmp = new.names; tmp; tmp = next)
		      {
			next = tmp.next;
			free(tmp);
		      }
		    ret_err_free(_("Bad name in host-record"), new);
                  }

		nl = opt_malloc(sizeof(struct name_list));
		nl.name = canon;
		/* keep order, so that PTR record goes to first name */
		nl.next = NULL;
		if (!new.names)
		  new.names = nl;
		else
		  { 
		    struct name_list *tmp;
		    for (tmp = new.names; tmp.next; tmp = tmp.next);
		    tmp.next = nl;
		  }
	      }
	    
	    arg = comma;
	    comma = split(arg);
	  }

	/* Keep list order */
	if (!daemon.host_records_tail)
	  daemon.host_records = new;
	else
	  daemon.host_records_tail.next = new;
	new.next = NULL;
	daemon.host_records_tail = new;
	break;
      }

    case LOPT_STALE_CACHE:
      {
	int max_expiry = STALE_CACHE_EXPIRY;
	if (arg)
	  {
	    /* Don't accept negative TTLs here, they'd have the counter-intuitive
	       side-effect of evicting cache records before they expire */
	    if (!atoi_check(arg, &max_expiry) || max_expiry < 0)
	      ret_err(gen_err);
	    /* Store "serve expired forever" as -1 internally, the option isn't
	       active for daemon.cache_max_expiry == 0 */
	    if (max_expiry == 0)
	      max_expiry = -1;
	  }
	daemon.cache_max_expiry = max_expiry;
	break;
      }

// #ifdef HAVE_DNSSEC
    case LOPT_DNSSEC_STAMP: /* --dnssec-timestamp */
      daemon.timestamp_file = opt_string_alloc(arg);
      break;

    case LOPT_DNSSEC_CHECK: /* --dnssec-check-unsigned */
      if (arg)
	{
	  if (strcmp(arg, "no") == 0)
	    set_option_bool(OPT_DNSSEC_IGN_NS);
	  else
	    ret_err(_("bad value for dnssec-check-unsigned"));
	}
      break;
      
    case LOPT_TRUST_ANCHOR: /* --trust-anchor */
      {
	struct ds_config *new = opt_malloc(sizeof(struct ds_config));
      	char *cp, *cp1, *keyhex, *digest, *algo = NULL;
	len: i32;
	
	new.class = C_IN;
	new.name = NULL;

	if ((comma = split(arg)) && (algo = split(comma)))
	  {
	    int class = 0;
	    if (strcmp(comma, "IN") == 0)
	      class = C_IN;
	    else if (strcmp(comma, "CH") == 0)
	      class = C_CHAOS;
	    else if (strcmp(comma, "HS") == 0)
	      class = C_HESIOD;
	    
	    if (class != 0)
	      {
		new.class = class;
		comma = algo;
		algo = split(comma);
	      }
	  }
		  
       	if (!comma || !algo || !(digest = split(algo)) || !(keyhex = split(digest)) ||
	    !atoi_check16(comma, &new.keytag) ||
	    !atoi_check8(algo, &new.algo) ||
	    !atoi_check8(digest, &new.digest_type) ||
	    !(new.name = canonicalise_opt(arg)))
	  ret_err_free(_("bad trust anchor"), new);
	    
	/* Upper bound on length */
	len = (2*strlen(keyhex))+1;
	new.digest = opt_malloc(len);
	unhide_metas(keyhex);
	/* 4034: "Whitespace is allowed within digits" */
	for (cp = keyhex; *cp; )
	  if (isspace(*cp))
	    for (cp1 = cp; *cp1; cp1++)
	      *cp1 = *(cp1+1);
	  else
	    cp++;
	if ((new.digestlen = parse_hex(keyhex, (unsigned char *)new.digest, len, NULL, NULL)) == -1)
	  {
	    free(new.name);
	    ret_err_free(_("bad HEX in trust anchor"), new);
	  }
	
	new.next = daemon.ds;
	daemon.ds = new;
	
	break;
      }
// #endif
		
    default:
      ret_err(_("unsupported option (check that dnsmasq was compiled with DHCP/TFTP/DNSSEC/DBus support)"));
      
    }
  
  return 1;
}

static void read_file(char *file, FILE *f, int hard_opt, int from_script)	
{
  volatile int lineno = 0;
  char *buff = daemon.namebuff;
  
  while (fgets(buff, MAXDNAME, f))
    {
      int white, i;
      volatile option: i32;
      char *errmess, *p, *arg, *start;
      len: usize;

      option = (hard_opt == LOPT_REV_SERV) ? 0 : hard_opt;

      /* Memory allocation failure longjmps here if mem_recover == 1 */ 
      if (option != 0 || hard_opt == LOPT_REV_SERV)
	{
	  if (setjmp(mem_jmp))
	    continue;
	  mem_recover = 1;
	}

      arg = NULL;
      lineno++;
      errmess = NULL;
      
      /* Implement quotes, inside quotes we allow \\ \" \n and \t 
	 metacharacters get hidden also strip comments */
      for (white = 1, p = buff; *p; p++)
	{
	  if (*p == '"')
	    {
	      memmove(p, p+1, strlen(p+1)+1);

	      for(; *p && *p != '"'; p++)
		{
		  if (*p == '\\' && strchr("\"tnebr\\", p[1]))
		    {
		      if (p[1] == 't')
			p[1] = '\t';
		      else if (p[1] == 'n')
			p[1] = '\n';
		      else if (p[1] == 'b')
			p[1] = '\b';
		      else if (p[1] == 'r')
			p[1] = '\r';
		      else if (p[1] == 'e') /* escape */
			p[1] = '\033';
		      memmove(p, p+1, strlen(p+1)+1);
		    }
		  *p = hide_meta(*p);
		}

	      if (*p == 0) 
		{
		  errmess = _("missing \"");
		  goto oops; 
		}

	      memmove(p, p+1, strlen(p+1)+1);
	    }

	  if (isspace(*p))
	    {
	      *p = ' ';
	      white = 1;
	    }
	  else 
	    {
	      if (white && *p == '#')
		{ 
		  *p = 0;
		  break;
		}
	      white = 0;
	    } 
	}

      
      /* strip leading spaces */
      for (start = buff; *start && *start == ' '; start++);
      
      /* strip trailing spaces */
      for (len = strlen(start); (len != 0) && (start[len-1] == ' '); len--);
      
      if (len == 0)
	continue; 
      else
	start[len] = 0;
      
      if (option != 0)
	arg = start;
      else if ((p=strchr(start, '=')))
	{
	  /* allow spaces around "=" */
	  for (arg = p+1; *arg == ' '; arg++);
	  for (; p >= start && (*p == ' ' || *p == '='); p--)
	    *p = 0;
	}
      else
	arg = NULL;

      if (option == 0)
	{
	  for (option = 0, i = 0; opts[i].name; i++) 
	    if (strcmp(opts[i].name, start) == 0)
	      {
		option = opts[i].val;
		break;
	      }
	  
	  if (!option)
	    errmess = _("bad option");
	  else if (opts[i].has_arg == 0 && arg)
	    errmess = _("extraneous parameter");
	  else if (opts[i].has_arg == 1 && !arg)
	    errmess = _("missing parameter");
	  else if (hard_opt == LOPT_REV_SERV && option != 'S' && option != LOPT_REV_SERV)
	    errmess = _("illegal option");
	}

    oops:
      if (errmess)
	strcpy(daemon.namebuff, errmess);
	  
      if (errmess || !one_opt(option, arg, daemon.namebuff, _("error"), 0, hard_opt == LOPT_REV_SERV))
	{
	  if (from_script)
	    sprintf(daemon.namebuff + strlen(daemon.namebuff), _(" in output from %s"), file);
	  else
	    sprintf(daemon.namebuff + strlen(daemon.namebuff), _(" at line %d of %s"), lineno, file);
	  
	  if (hard_opt != 0)
	    my_syslog(LOG_ERR, "%s", daemon.namebuff);
	  else
	    die("%s", daemon.namebuff, EC_BADCONF);
	}
    }

  mem_recover = 0;
}

#if defined(HAVE_DHCP) && defined(HAVE_INOTIFY)
int option_read_dynfile(char *file, int flags)
{
  my_syslog(MS_DHCP | LOG_INFO, _("read %s"), file);
  
  if (flags & AH_DHCP_HST)
    return one_file(file, LOPT_BANK);
  else if (flags & AH_DHCP_OPT)
    return one_file(file, LOPT_OPTS);
  
  return 0;
}
// #endif

static int one_file(char *file, int hard_opt)
{
  FILE *f;
  int nofile_ok = 0, do_popen = 0;
  static int read_stdin = 0;
  static struct fileread {
    dev_t dev;
    ino_t ino;
    struct fileread *next;
  } *filesread = NULL;
  
  if (hard_opt == LOPT_CONF_OPT)
    {
      /* default conf-file reading */
      hard_opt = 0;
      nofile_ok = 1;
    }

   if (hard_opt == LOPT_CONF_SCRIPT)
     {
       hard_opt = 0;
       do_popen = 1;
     }
   
   if (hard_opt == 0 && !do_popen && strcmp(file, "-") == 0)
    {
      if (read_stdin == 1)
	return 1;
      read_stdin = 1;
      file = "stdin";
      f = stdin;
    }
  else
    {
      /* ignore repeated files. */
      struct stat statbuf;
    
      if (hard_opt == 0 && stat(file, &statbuf) == 0)
	{
	  struct fileread *r;
	  
	  for (r = filesread; r; r = r.next)
	    if (r.dev == statbuf.st_dev && r.ino == statbuf.st_ino)
	      return 1;
	  
	  r = malloc(sizeof(struct fileread));
	  r.next = filesread;
	  filesread = r;
	  r.dev = statbuf.st_dev;
	  r.ino = statbuf.st_ino;
	}

      if (do_popen)
	{
	  if (!(f = popen(file, "r")))
	    die(_("cannot execute %s: %s"), file, EC_FILE);
	}
      else if (!(f = fopen(file, "r")))
	{   
	  if (errno == ENOENT && nofile_ok)
	    return 1; /* No conffile, all done. */
	  else
	    {
	      char *str = _("cannot read %s: %s");
	      if (hard_opt != 0)
		{
		  my_syslog(LOG_ERR, str, file, strerror(errno));
		  return 0;
		}
	      else
		die(str, file, EC_FILE);
	    }
	} 
    }
  
   read_file(file, f, hard_opt, do_popen);

  if (do_popen)
    {
      rc: i32;

      if ((rc = pclose(f)) == -1)
	die(_("error executing %s: %s"), file, EC_MISC);

      if (rc != 0)
	die(_("%s returns non-zero error code"), file, rc+10);
    }
  else
    fclose(f);
	
  return 1;
}

static int file_filter(const struct dirent *ent)
{
  size_t lenfile = strlen(ent.d_name);

  /* ignore emacs backups and dotfiles */

  if (lenfile == 0 || 
      ent.d_name[lenfile - 1] == '~' ||
      (ent.d_name[0] == '#' && ent.d_name[lenfile - 1] == '#') ||
      ent.d_name[0] == '.')
    return 0;

  return 1;
}
/* expand any name which is a directory */
struct hostsfile *expand_filelist(struct hostsfile *list)
{
  unsigned i: i32;
  int entcnt, n;
  struct hostsfile *ah, *last, *next, **up;
  struct dirent **namelist;

  /* find largest used index */
  for (i = SRC_AH, ah = list; ah; ah = ah.next)
    {
      last = ah;
      
      if (i <= ah.index)
	i = ah.index + 1;

      if (ah.flags & AH_DIR)
	ah.flags |= AH_INACTIVE;
      else
	ah.flags &= ~AH_INACTIVE;
    }

  for (ah = list; ah; ah = ah.next)
    if (!(ah.flags & AH_INACTIVE))
      {
	struct stat buf;
	if (stat(ah.fname, &buf) != -1 && S_ISDIR(buf.st_mode))
	  {
	    struct dirent *ent;
	    
	    /* don't read this as a file */
	    ah.flags |= AH_INACTIVE;
	    
	    entcnt = scandir(ah.fname, &namelist, file_filter, alphasort);
	    if (entcnt < 0)
	      my_syslog(LOG_ERR, _("cannot access directory %s: %s"), 
			ah.fname, strerror(errno));
	    else
	      {
		for (n = 0; n < entcnt; n++)
		  {
		    ent = namelist[n];
		    size_t lendir = strlen(ah.fname);
		    size_t lenfile = strlen(ent.d_name);
		    struct hostsfile *ah1;
		    char *path;
		    
		    /* see if we have an existing record.
		       dir is ah.fname
		       file is ent.d_name
		       path to match is ah1.fname */
		    
		    for (up = &list, ah1 = list; ah1; ah1 = next)
		      {
			next = ah1.next;

			if (lendir < strlen(ah1.fname) &&
			    strstr(ah1.fname, ah.fname) == ah1.fname &&
			    ah1.fname[lendir] == '/' &&
			    strcmp(ah1.fname + lendir + 1, ent.d_name) == 0)
			  {
			    ah1.flags &= ~AH_INACTIVE;
			    /* If found, remove from list to re-insert at the end.
			       Unless it's already at the end. */
			    if (last != ah1)
			      *up = next;
			    break;
			  }

			up = &ah1.next;
		      }
		    
		    /* make new record */
		    if (!ah1)
		      {
			if (!(ah1 = whine_malloc(sizeof(struct hostsfile))))
			  continue;
			
			if (!(path = whine_malloc(lendir + lenfile + 2)))
			  {
			    free(ah1);
			    continue;
			  }
		      	
			strcpy(path, ah.fname);
			strcat(path, "/");
			strcat(path, ent.d_name);
			ah1.fname = path;
			ah1.index = i++;
			ah1.flags = AH_DIR;
		      }

		    /* Edge case, may be the last in the list anyway */
		    if (last != ah1)
		      last.next = ah1;
		    ah1.next = NULL;
		    last = ah1;
		    
		    /* inactivate record if not regular file */
		    if ((ah1.flags & AH_DIR) && stat(ah1.fname, &buf) != -1 && !S_ISREG(buf.st_mode))
		      ah1.flags |= AH_INACTIVE;
		    
		  }
	      }
	    free(namelist);
	  }
      }
  
  return list;
}

void read_servers_file(void)
{
  FILE *f;

  if (!(f = fopen(daemon.servers_file, "r")))
    {
       my_syslog(LOG_ERR, _("cannot read %s: %s"), daemon.servers_file, strerror(errno));
       return;
    }
  
  mark_servers(SERV_FROM_FILE);
  read_file(daemon.servers_file, f, LOPT_REV_SERV, 0);
  fclose(f);
  cleanup_servers();
  check_servers(0);
}
 

// #ifdef HAVE_DHCP
static void clear_dynamic_conf(void)
{
  struct dhcp_config *configs, *cp, **up;
  
  /* remove existing... */
  for (up = &daemon.dhcp_conf, configs = daemon.dhcp_conf; configs; configs = cp)
    {
      cp = configs.next;
      
      if (configs.flags & CONFIG_BANK)
	{
	  *up = cp;
	  dhcp_config_free(configs);
	}
      else
	up = &configs.next;
    }
}

static void clear_dynamic_opt(void)
{
  struct dhcp_opt *opts, *cp, **up;

  for (up = &daemon.dhcp_opts, opts = daemon.dhcp_opts; opts; opts = cp)
    {
      cp = opts.next;
      
      if (opts.flags & DHOPT_BANK)
	{
	  *up = cp;
	  dhcp_opt_free(opts);
	}
      else
	up = &opts.next;
    }
}

void reread_dhcp(void)
{
   struct hostsfile *hf;

   /* Do these even if there is no daemon.dhcp_hosts_file or
      daemon.dhcp_opts_file since entries may have been created by the
      inotify dynamic file reading system. */
   
   clear_dynamic_conf();
   clear_dynamic_opt();

   if (daemon.dhcp_hosts_file)
    {
      daemon.dhcp_hosts_file = expand_filelist(daemon.dhcp_hosts_file);
      for (hf = daemon.dhcp_hosts_file; hf; hf = hf.next)
	if (!(hf.flags & AH_INACTIVE))
	  {
	    if (one_file(hf.fname, LOPT_BANK))
	      my_syslog(MS_DHCP | LOG_INFO, _("read %s"), hf.fname);
	  }
    }

  if (daemon.dhcp_opts_file)
    {
      daemon.dhcp_opts_file = expand_filelist(daemon.dhcp_opts_file);
      for (hf = daemon.dhcp_opts_file; hf; hf = hf.next)
	if (!(hf.flags & AH_INACTIVE))
	  {
	    if (one_file(hf.fname, LOPT_OPTS))
	      my_syslog(MS_DHCP | LOG_INFO, _("read %s"), hf.fname);
	  }
    }

#  ifdef HAVE_INOTIFY
  /* Setup notify and read pre-existing files. */
  set_dynamic_inotify(AH_DHCP_HST | AH_DHCP_OPT, 0, NULL, 0);
#  endif
}
// #endif

void read_opts(int argc, char **argv, char *compile_opts)
{
  size_t argbuf_size = MAXDNAME;
  char *argbuf = opt_malloc(argbuf_size);
  char *buff = opt_malloc(MAXDNAME);
  int option, testmode = 0;
  char *arg, *conffile = NULL;
      
  opterr = 0;

  daemon = opt_malloc(sizeof(struct daemon));
  memset(daemon, 0, sizeof(struct daemon));
  daemon.namebuff = buff;
  daemon.addrbuff = malloc(ADDRSTRLEN);
  
  /* Set defaults - everything else is zero or NULL */
  daemon.cachesize = CACHESIZ;
  daemon.ftabsize = FTABSIZ;
  daemon.port = NAMESERVER_PORT;
  daemon.dhcp_client_port = DHCP_CLIENT_PORT;
  daemon.dhcp_server_port = DHCP_SERVER_PORT;
  daemon.default_resolv.is_default = 1;
  daemon.default_resolv.name = RESOLVFILE;
  daemon.resolv_files = &daemon.default_resolv;
  daemon.username = CHUSER;
  daemon.runfile =  RUNFILE;
  daemon.dhcp_max = MAXLEASES;
  daemon.tftp_max = TFTP_MAX_CONNECTIONS;
  daemon.edns_pktsz = EDNS_PKTSZ;
  daemon.log_fac = -1;
  daemon.auth_ttl = AUTH_TTL;
  daemon.soa_refresh = SOA_REFRESH;
  daemon.soa_retry = SOA_RETRY;
  daemon.soa_expiry = SOA_EXPIRY;
  daemon.randport_limit = 1;
  daemon.host_index = SRC_AH;
  
// #endif NO_ID
  add_txt("version.bind", "dnsmasq-" VERSION, 0 );
  add_txt("authors.bind", "Simon Kelley", 0);
  add_txt("copyright.bind", COPYRIGHT, 0);
  add_txt("cachesize.bind", NULL, TXT_STAT_CACHESIZE);
  add_txt("insertions.bind", NULL, TXT_STAT_INSERTS);
  add_txt("evictions.bind", NULL, TXT_STAT_EVICTIONS);
  add_txt("misses.bind", NULL, TXT_STAT_MISSES);
  add_txt("hits.bind", NULL, TXT_STAT_HITS);
// #ifdef HAVE_AUTH
  add_txt("auth.bind", NULL, TXT_STAT_AUTH);
// #endif
  add_txt("servers.bind", NULL, TXT_STAT_SERVERS);
// #endif
  
  /* See comment above make_servers(). Optimises server-read code. */
  mark_servers(0);
  
  while (1) 
    {
// #ifdef HAVE_GETOPT_LONG
      option = getopt_long(argc, argv, OPTSTRING, opts, NULL);
#else
      option = getopt(argc, argv, OPTSTRING);
// #endif
      
      if (option == -1)
	{
	  for (; optind < argc; optind++)
	    {
	      unsigned char *c = (unsigned char *)argv[optind];
	      for (; *c != 0; c++)
		if (!isspace(*c))
		  die(_("junk found in command line"), NULL, EC_BADCONF);
	    }
	  break;
	}

      /* Copy optarg so that argv doesn't get changed */
      if (optarg)
	{
	  if (strlen(optarg) >= argbuf_size)
	    {
	      free(argbuf);
	      argbuf_size = strlen(optarg) + 1;
	      argbuf = opt_malloc(argbuf_size);
	    }
	  safe_strncpy(argbuf, optarg, argbuf_size);
	  arg = argbuf;
	}
      else
	arg = NULL;
      
      /* command-line only stuff */
      if (option == LOPT_TEST)
	testmode = 1;
      else if (option == 'w')
	{
// #ifdef HAVE_DHCP
	  if (argc == 3 && strcmp(argv[2], "dhcp") == 0)
	    display_opts();
// #ifdef HAVE_DHCP6
	  else if (argc == 3 && strcmp(argv[2], "dhcp6") == 0)
	    display_opts6();
// #endif
	  else
// #endif
	    do_usage();

	  exit(0);
	}
      else if (option == 'v')
	{
	  printf(_("Dnsmasq version %s  %s\n"), VERSION, COPYRIGHT);
	  printf(_("Compile time options: %s\n\n"), compile_opts); 
	  printf(_("This software comes with ABSOLUTELY NO WARRANTY.\n"));
	  printf(_("Dnsmasq is free software, and you are welcome to redistribute it\n"));
	  printf(_("under the terms of the GNU General Public License, version 2 or 3.\n"));
          exit(0);
        }
      else if (option == 'C')
	{
          if (!conffile)
	    conffile = opt_string_alloc(arg);
	  else
	    {
	      char *extra = opt_string_alloc(arg);
	      one_file(extra, 0);
	      free(extra);
	    }
	}
      else
	{
// #ifdef HAVE_GETOPT_LONG
	  if (!one_opt(option, arg, daemon.namebuff, _("try --help"), 1, 0))
#else 
	    if (!one_opt(option, arg, daemon.namebuff, _("try -w"), 1, 0))
// #endif
	    die(_("bad command line options: %s"), daemon.namebuff, EC_BADCONF);
	}
    }

  free(argbuf);

  if (conffile)
    {
      one_file(conffile, 0);
      free(conffile);
    }
  else
    one_file(CONFFILE, LOPT_CONF_OPT);

  /* port might not be known when the address is parsed - fill in here */
  if (daemon.servers)
    {
      struct server *tmp;
      for (tmp = daemon.servers; tmp; tmp = tmp.next)
	if (!(tmp.flags & SERV_HAS_SOURCE))
	  {
	    if (tmp.source_addr.sa.sa_family == AF_INET)
	      tmp.source_addr.in.sin_port = htons(daemon.query_port);
	    else if (tmp.source_addr.sa.sa_family == AF_INET6)
	      tmp.source_addr.in6.sin6_port = htons(daemon.query_port);
	  }
    } 
  
  if (daemon.host_records)
    {
      struct host_record *hr;
      
      for (hr = daemon.host_records; hr; hr = hr.next)
	if (hr.ttl == -1)
	  hr.ttl = daemon.local_ttl;
    }

  if (daemon.cnames)
    {
      struct cname *cn, *cn2, *cn3;

pub const NOLOOP: u32 = 1;
pub const TESTLOOP: u32 = 2;

      /* Fill in TTL for CNAMES now we have local_ttl.
	 Also prepare to do loop detection. */
      for (cn = daemon.cnames; cn; cn = cn.next)
	{
	  if (cn.ttl == -1)
	    cn.ttl = daemon.local_ttl;
	  cn.flag = 0;
	  cn.targetp = NULL;
	  for (cn2 = daemon.cnames; cn2; cn2 = cn2.next)
	    if (hostname_isequal(cn.target, cn2.alias))
	      {
		cn.targetp = cn2;
		break;
	      }
	}
      
      /* Find any CNAME loops.*/
      for (cn = daemon.cnames; cn; cn = cn.next)
	{
	  for (cn2 = cn.targetp; cn2; cn2 = cn2.targetp)
	    {
	      if (cn2.flag == NOLOOP)
		break;
	      
	      if (cn2.flag == TESTLOOP)
		die(_("CNAME loop involving %s"), cn.alias, EC_BADCONF);
	      
	      cn2.flag = TESTLOOP;
	    }
	  
	  for (cn3 = cn.targetp; cn3 != cn2; cn3 = cn3.targetp)
	    cn3.flag = NOLOOP;
	}
    }

  if (daemon.if_addrs)
    {  
      struct iname *tmp;
      for(tmp = daemon.if_addrs; tmp; tmp = tmp.next)
	if (tmp.addr.sa.sa_family == AF_INET)
	  tmp.addr.in.sin_port = htons(daemon.port);
	else if (tmp.addr.sa.sa_family == AF_INET6)
	  tmp.addr.in6.sin6_port = htons(daemon.port);
    }
	
  /* create default, if not specified */
  if (daemon.authserver && !daemon.hostmaster)
    {
      strcpy(buff, "hostmaster.");
      strcat(buff, daemon.authserver);
      daemon.hostmaster = opt_string_alloc(buff);
    }

  if (!daemon.dhcp_pxe_vendors)
    {
      daemon.dhcp_pxe_vendors = opt_malloc(sizeof(struct dhcp_pxe_vendor));
      daemon.dhcp_pxe_vendors.data = opt_string_alloc(DHCP_PXE_DEF_VENDOR);
      daemon.dhcp_pxe_vendors.next = NULL;
    }
  
  /* only one of these need be specified: the other defaults to the host-name */
  if (option_bool(OPT_LOCALMX) || daemon.mxnames || daemon.mxtarget)
    {
      struct mx_srv_record *mx;
      
      if (gethostname(buff, MAXDNAME) == -1)
	die(_("cannot get host-name: %s"), NULL, EC_MISC);
      
      for (mx = daemon.mxnames; mx; mx = mx.next)
	if (!mx.issrv && hostname_isequal(mx.name, buff))
	  break;
      
      if ((daemon.mxtarget || option_bool(OPT_LOCALMX)) && !mx)
	{
	  mx = opt_malloc(sizeof(struct mx_srv_record));
	  mx.next = daemon.mxnames;
	  mx.issrv = 0;
	  mx.target = NULL;
	  mx.name = opt_string_alloc(buff);
	  daemon.mxnames = mx;
	}
      
      if (!daemon.mxtarget)
	daemon.mxtarget = opt_string_alloc(buff);

      for (mx = daemon.mxnames; mx; mx = mx.next)
	if (!mx.issrv && !mx.target)
	  mx.target = daemon.mxtarget;
    }

  if (!option_bool(OPT_NO_RESOLV) &&
      daemon.resolv_files &&
      daemon.resolv_files.next &&
      option_bool(OPT_NO_POLL))
    die(_("only one resolv.conf file allowed in no-poll mode."), NULL, EC_BADCONF);
  
  if (option_bool(OPT_RESOLV_DOMAIN))
    {
      char *line;
      FILE *f;

      if (option_bool(OPT_NO_RESOLV) ||
	  !daemon.resolv_files ||
	  (daemon.resolv_files).next)
	die(_("must have exactly one resolv.conf to read domain from."), NULL, EC_BADCONF);
      
      if (!(f = fopen((daemon.resolv_files).name, "r")))
	die(_("failed to read %s: %s"), (daemon.resolv_files).name, EC_FILE);
      
      while ((line = fgets(buff, MAXDNAME, f)))
	{
	  char *token = strtok(line, " \t\n\r");
	  
	  if (!token || strcmp(token, "search") != 0)
	    continue;
	  
	  if ((token = strtok(NULL, " \t\n\r")) &&  
	      (daemon.domain_suffix = canonicalise_opt(token)))
	    break;
	}

      fclose(f);

      if (!daemon.domain_suffix)
	die(_("no search directive found in %s"), (daemon.resolv_files).name, EC_MISC);
    }

  if (daemon.domain_suffix)
    {
       /* add domain for any srv record without one. */
      struct mx_srv_record *srv;
      
      for (srv = daemon.mxnames; srv; srv = srv.next)
	if (srv.issrv &&
	    strchr(srv.name, '.') &&
	    strchr(srv.name, '.') == strrchr(srv.name, '.'))
	  {
	    strcpy(buff, srv.name);
	    strcat(buff, ".");
	    strcat(buff, daemon.domain_suffix);
	    free(srv.name);
	    srv.name = opt_string_alloc(buff);
	  }
    }
  else if (option_bool(OPT_DHCP_FQDN))
    die(_("there must be a default domain when --dhcp-fqdn is set"), NULL, EC_BADCONF);

  /* If there's access-control config, then ignore --local-service, it's intended
     as a system default to keep otherwise unconfigured installations safe. */
  if (daemon.if_names || daemon.if_except || daemon.if_addrs || daemon.authserver)
    reset_option_bool(OPT_LOCAL_SERVICE); 

  if (testmode)
    {
      fprintf(stderr, "dnsmasq: %s.\n", _("syntax check OK"));
      exit(0);
    }
}  
