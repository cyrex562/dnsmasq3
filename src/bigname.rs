#[derive(Debug,Clone)]
pub union bigname {
  name: [u8;MAXDNAME],
  // union bigname *next; /* freelist */
};
