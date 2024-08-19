use ocl::ProQue;

use crate::{
    config::{Settings, MAX_SIZE_X, MAX_SIZE_Y},
    simulation::TrailMap,
};

pub fn gpu_diffuse(trail_map: &mut TrailMap, settings: &Settings) -> ocl::Result<()> {
    let kernel = r#"
        __kernel void diffuse(__global double* trailmap, double trail_diffuse, long max_size_x, long max_size_y) {
            int x = get_global_id(0) % max_size_x;
            int y = get_global_id(0) / max_size_x;
            double sum = trailmap[get_global_id(0)];

            // Top Row
            // Bottom Row
            // Right Column
            // Left Column

            // Middle
            if ( (x != 0) && ( y != 0) && (x < (max_size_x-1)) && (y < (max_size_y-1)) ) {
                sum += trailmap[x + 1 + (y-1) * max_size_x];
                sum += trailmap[x + 1 + y * max_size_x];
                sum += trailmap[x + 1 + (y+1) * max_size_x];

                sum += trailmap[x + (y-1) * max_size_x];
                sum += trailmap[x + (y+1) * max_size_x];

                sum += trailmap[x - 1 + (y-1) * max_size_x];
                sum += trailmap[x - 1 + y * max_size_x];
                sum += trailmap[x - 1 + (y+1) * max_size_x];

                trailmap[get_global_id(0)] *= (double)1 - trail_diffuse;
                trailmap[get_global_id(0)] += sum / (double)9 * trail_diffuse;
            }
        }
    "#;

    let pro_que = ProQue::builder()
        .src(kernel)
        .dims(MAX_SIZE_X * MAX_SIZE_Y)
        .build()?;

    let buffer = pro_que.create_buffer::<f64>()?;
    buffer.write(&*trail_map).enq()?;

    let kernel = pro_que
        .kernel_builder("diffuse")
        .arg(&buffer)
        .arg(settings.trail_diffuse)
        .arg(MAX_SIZE_X as i64)
        .arg(MAX_SIZE_Y as i64)
        .build()?;

    unsafe {
        kernel.enq()?;
    }

    buffer.read(trail_map).enq()?;
    Ok(())
}


pub fn gpu_decay(trail_map: &mut TrailMap, settings: &Settings) -> ocl::Result<()> {
    let kernel = r#"
        __kernel void decay(__global double* trailmap, double trail_decay) {
            if (trailmap[get_global_id(0)] < trail_decay) {
                trailmap[get_global_id(0)] = 0;
            } else {
                trailmap[get_global_id(0)] -= trail_decay;
            }
        }
    "#;

    let pro_que = ProQue::builder()
        .src(kernel)
        .dims(MAX_SIZE_X * MAX_SIZE_Y)
        .build()?;

    let buffer = pro_que.create_buffer::<f64>()?;
    buffer.write(&*trail_map).enq()?;

    let kernel = pro_que
        .kernel_builder("decay")
        .arg(&buffer)
        .arg(settings.trail_decay)
        .build()?;

    unsafe {
        kernel.enq()?;
    }

    buffer.read(trail_map).enq()?;
    Ok(())
}
