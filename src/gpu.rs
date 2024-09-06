use ocl::ProQue;

use crate::{
    config::{Settings, MAX_AGENT_N, MAX_SIZE_X, MAX_SIZE_Y},
    simulation::{Agent, Agents, TrailMap},
};

pub fn gpu_move(agents: &mut Agents, settings: &Settings) -> ocl::Result<()> {
    let kernel = r#"

        struct agent {
            double pos_x;
            double pos_y;
            double angle;
        };

        __kernel void move(__global struct agent* one_agent, double agent_speed, uint agent_n, uint size_x, uint size_y) {
            if (get_global_id(0) < agent_n) {

                one_agent->pos_x += cos(one_agent->angle) * agent_speed;
                one_agent->pos_y += sin(one_agent->angle) * agent_speed;

                // Check Collision
                if ( (one_agent->pos_x < (double)0) ||
                     (one_agent->pos_x >= (double)size_x) ||
                     (one_agent->pos_y < (double)0) ||
                     (one_agent->pos_y >= (double)size_y)
                    ) {
                    // one_agent->pos_x = one_agent->pos_x.max(0_f64).min(size_x - 1);
                    // one_agent->pos_y = one_agent->pos_y.max(0_f64).min(size_y - 1);
                    // one_agent->angle = rng.gen::<f64>() * 2_f64 * PI;
                }
            }
        }
    "#;

    let pro_que = ProQue::builder()
        .src(kernel)
        .dims(MAX_AGENT_N)
        .build()?;

    let buffer = pro_que.create_buffer::<Agent>()?;
    buffer.write(&*agents).enq()?;

    let kernel = pro_que
        .kernel_builder("move")
        .arg(&buffer)
        .arg(settings.agent_speed)
        .arg(settings.agent_n)
        .arg(settings.size_x)
        .arg(settings.size_y)
        .build()?;

    unsafe {
        kernel.enq()?;
    }

    buffer.read(agents).enq()?;
    Ok(())
}

pub fn gpu_diffuse(trail_map: &mut TrailMap, settings: &Settings) -> ocl::Result<()> {
    let kernel = r#"
        __kernel void diffuse(__global double* trailmap, double trail_diffuse, uint max_size_x, uint max_size_y) {
            int x = get_global_id(0) % max_size_x;
            int y = get_global_id(0) / max_size_x;
            double sum = 0;

            for (int offset_X = -1; offset_X <= 1; offset_X++) {
            for (int offset_Y = -1; offset_Y <= 1; offset_Y++) {

                int p_x = x + offset_X;
                int p_y = y + offset_Y;

            if ( p_x >= 0 && p_y >= 0 && p_x < max_size_x && p_y < max_size_y ) {
                sum += trailmap[p_x + p_y * max_size_x];


            }

            }}
            trailmap[get_global_id(0)] *= (double)1 - trail_diffuse;
            trailmap[get_global_id(0)] += sum / (double)9 * trail_diffuse;
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
        .arg(MAX_SIZE_X)
        .arg(MAX_SIZE_Y)
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

pub fn gpu_all(trail_map: &mut TrailMap, settings: &Settings) -> ocl::Result<()> {
    let kernel = r#"
            __kernel void all(__global double* trailmap, uint max_size_x, uint max_size_y, double trail_diffuse, double trail_decay) {

            int x = get_global_id(0) % max_size_x;
            int y = get_global_id(0) / max_size_x;
            double sum = 0;

            for (int offset_X = -1; offset_X <= 1; offset_X++) {
            for (int offset_Y = -1; offset_Y <= 1; offset_Y++) {

                int p_x = x + offset_X;
                int p_y = y + offset_Y;

            if ( p_x >= 0 && p_y >= 0 && p_x < max_size_x && p_y < max_size_y ) {
                sum += trailmap[p_x + p_y * max_size_x];


            }

            }}
            trailmap[get_global_id(0)] *= (double)1 - trail_diffuse;
            trailmap[get_global_id(0)] += sum / (double)9 * trail_diffuse;


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
        .kernel_builder("all")
        .arg(&buffer)
        .arg(MAX_SIZE_X)
        .arg(MAX_SIZE_Y)
        .arg(settings.trail_diffuse)
        .arg(settings.trail_decay)
        .build()?;

    unsafe {
        kernel.enq()?;
    }

    buffer.read(trail_map).enq()?;
    Ok(())
}
