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





pub fn loop_send_probes()
{
   let mut serv: server;
   
   if (!daemon.opt_loop_detect) {
     return;
   }

   /* Loop through all upstream servers not for particular domains, and send a query to that server which is
      identifiable, via the uid. If we see that query back again, then the server is looping, and we should not use it. */
  //  for (serv = daemon.servers; serv; serv = serv.next) 
  for serv in daemon.servers 
  {
     if (!(serv.flags & 
	   (SERV_LITERAL_ADDRESS | SERV_NO_ADDR | SERV_USE_RESOLV | SERV_NO_REBIND | SERV_HAS_DOMAIN | SERV_FOR_NODOTS | SERV_LOOP)))
       {
	 let slen: usize = loop_make_probe(serv.uid);
	 let mut fd: i32;
	 let rfd: randfd;
	 
	 if (serv.sfd) {
	   fd = serv.sfd.fd;
   }
	 else 
	   {
	     if (!(rfd = allocate_rfd(serv.addr.sa.sa_family))) {
	       continue;
       }
	     fd = rfd.fd;
	   }

	 while {retry_send(sendto(fd, daemon.packet, len, 0, 
				  &serv.addr.sa, sa_len(&serv.addr)))}{}
	 
	 free_rfd(rfd);
       }
      }
}
  
 sloop_make_probe: usize(u32 uid)
{
  struct dns_header *header = (struct dns_header *)daemon.packet;
  unsigned char *p = (header+1);

  /* packet buffer overwritten */
  daemon.srv_save = NULL;
  
  header.id = rand16();
  header.ancount = header.nscount = header.arcount = htons(0);
  header.qdcount = htons(1);
  header.hb3 = HB3_RD;
  header.hb4 = 0;
  SET_OPCODE(header, QUERY);

  *p++ = 8;
  sprintf(p, "%.8x", uid);
  p += 8;
  *p++ = strlen(LOOP_TEST_DOMAIN);
  strcpy(p, LOOP_TEST_DOMAIN); /* Add terminating zero */
  p += strlen(LOOP_TEST_DOMAIN) + 1;

  PUTSHORT(LOOP_TEST_TYPE, p);
  PUTSHORT(C_IN, p);

  return p - header;
}
  

int detect_loop(query: &mut String, type: i32)
{
  let mut i: i32;
  u32 uid;
  let mut serv: server;
  
  if (!daemon.opt_loop_detect)
    return 0;

  if (type != LOOP_TEST_TYPE ||
      strlen(LOOP_TEST_DOMAIN) + 9 != strlen(query) ||
      strstr(query, LOOP_TEST_DOMAIN) != query + 9)
    return 0;

  for i in 0..8
    if (!isxdigit(query[i]))
      return 0;

  uid = strtol(query, NULL, 16);

  for (serv = daemon.servers; serv; serv = serv.next)
     if (!(serv.flags & 
	   (SERV_LITERAL_ADDRESS | SERV_NO_ADDR | SERV_USE_RESOLV | SERV_NO_REBIND | SERV_HAS_DOMAIN | SERV_FOR_NODOTS | SERV_LOOP)) &&
	 uid == serv.uid)
       {
	 serv.flags |= SERV_LOOP;
	 check_servers(); /* log new state */
	 return 1;
       }

  return 0;
}


