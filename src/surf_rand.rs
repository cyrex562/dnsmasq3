
#[derive(Default, Debug, Clone)]
pub struct SurfRandContext {
    //
// static u32 seed[32];
// static u32 in[12];
// static u32 out[8];
// static int outleft = 0;
    pub seed: [u32; 32],
    pub input: [u32; 12],
    pub out: [u32; 8],
    pub outleft: i32,
}


pub fn rand_init(rand_ctx: &mut SurfRandContext) {
    let mut fd = unsafe { open(RANDFILE.into(), O_RDONLY) };

    if fd == -1 || !read_write(fd, &mut rand_ctx.seed, rand_ctx.seed.len(), 1) || !read_write(fd, &mut rand_ctx.input, rand_ctx.input.len(), 1) {
        panic!("failed to seed the random number generator");
    }

    unsafe { close(fd); }
}

#define ROTATE(x,b) (((x) << (b)) | ((x) >> (32 - (b))))
#define MUSH(i,b) x = t[i] += (((x ^ seed[i]) + sum) ^ ROTATE(x,b));

static void surf(void)
{
  u32 t[12]; u32 x; u32 sum = 0;
  r: i32; i: i32; loop: i32;

  for (i = 0;i < 12;++i) t[i] = in[i] ^ seed[12 + i];
  for (i = 0;i < 8;++i) out[i] = seed[24 + i];
  x = t[11];
  for (loop = 0;loop < 2;++loop) {
    for (r = 0;r < 16;++r) {
      sum += 0x9e3779b9;
      MUSH(0,5) MUSH(1,7) MUSH(2,9) MUSH(3,13)
      MUSH(4,5) MUSH(5,7) MUSH(6,9) MUSH(7,13)
      MUSH(8,5) MUSH(9,7) MUSH(10,9) MUSH(11,13)
    }
    for (i = 0;i < 8;++i) out[i] ^= t[i + 4];
  }
}

unsigned short rand16(void)
{
  if (!outleft)
    {
      if (!++in[0]) if (!++in[1]) if (!++in[2]) ++in[3];
      surf();
      outleft = 8;
    }

  return (unsigned short) out[--outleft];
}

u32 rand32(void)
{
 if (!outleft)
    {
      if (!++in[0]) if (!++in[1]) if (!++in[2]) ++in[3];
      surf();
      outleft = 8;
    }

  return out[--outleft];
}

u64 rand64(void)
{
  static int outleft = 0;

  if (outleft < 2)
    {
      if (!++in[0]) if (!++in[1]) if (!++in[2]) ++in[3];
      surf();
      outleft = 8;
    }

  outleft -= 2;

  return (u64)out[outleft+1] + (((u64)out[outleft]) << 32);
}
