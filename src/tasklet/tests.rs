use super::*;

#[test]
fn const_size() {
    type TaskletStub = Tasklet<(), ()>;
    let stub_size = core::mem::size_of::<TaskletStub>();

    struct NoCtx;
    type TaskletNoCtx = Tasklet<u8, NoCtx>;
    let noctx_size = core::mem::size_of::<TaskletNoCtx>();

    struct SmallCtx {
        _a: u16,
    }
    type TaskletSmallCtx = Tasklet<u16, SmallCtx>;
    let smallctx_size = core::mem::size_of::<TaskletSmallCtx>();

    struct BigCtx {
        _a: u64,
        _b: f64,
        _c: u16,
    }
    type TaskletBigCtx = Tasklet<u32, BigCtx>;
    let bigctx_size = core::mem::size_of::<TaskletBigCtx>();

    assert_eq!(noctx_size, stub_size);
    assert_eq!(smallctx_size, stub_size);
    assert_eq!(bigctx_size, stub_size);
}
