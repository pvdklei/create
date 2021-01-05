use peppaint::{
    MyContext,
    Movie,
    Actor,
};

use crate::{
    RingTunnel,
    FirstPersonRollerCoasterCam,
    ClosedTunnel,
    routes::*
};

#[allow(dead_code)]
pub struct TunnelSimulator {
    pooled_tunnel: RingTunnel,
    closed_tunnel: ClosedTunnel,
    camera: FirstPersonRollerCoasterCam, 
}

impl Movie for TunnelSimulator {
    
    fn setup(ctx: &mut MyContext) -> Self {

        let camera = crate::FirstPersonRollerCoasterCam::ne();

        let f = f2;

        ctx.set_route(f);

        let pooled_tunnel = RingTunnel::ne_def(f);
        let closed_tunnel = ClosedTunnel::ne_def(f);

        Self { 
            pooled_tunnel,
            closed_tunnel,
            camera,
        }
    }

    fn update(&mut self, ctx: &mut MyContext) {
        self.camera.act(ctx);
        // self.closed_tunnel.act(ctx);
        self.pooled_tunnel.act(ctx);
    }
}

