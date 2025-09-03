// Game of life compute

struct Settings {
    rng_seed: u32,
}

@group(0) @binding(0) var input: texture_storage_2d<rgba32float, read>;
@group(0) @binding(1) var output: texture_storage_2d<rgba32float, write>;
@group(0) @binding(2) var<uniform> settings: Settings;

fn hash(value: u32) -> u32 {
    var state = value;
    state += settings.rng_seed;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn random_float_aa(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

fn random_float_bb(value: u32) -> f32 {
    return f32(hash(hash(value))) / 4294967295.0;
}

fn is_alive(location: vec2<i32>, offset_x: i32, offset_y: i32, index: u32) -> u32 {
    let value: vec4<f32> = textureLoad(input, location + vec2<i32>(offset_x, offset_y));
    return u32(value[index]);
}

fn count_alive_neighbors(location: vec2<i32>, index: u32) -> u32 {
    return
        is_alive(location, -1, -1, index) +
        is_alive(location, -1,  0, index) +
        is_alive(location, -1,  1, index) +
        is_alive(location,  0, -1, index) +
        is_alive(location,  0,  1, index) +
        is_alive(location,  1, -1, index) +
        is_alive(location,  1,  0, index) +
        is_alive(location,  1,  1, index);
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let aa = random_float_aa(invocation_id.y << 16u | invocation_id.x);
    let bb = random_float_bb(invocation_id.y << 16u | invocation_id.x);
    let aa_alive = aa > 0.9;
    let bb_alive = bb > 0.9;
    let cc_alive = aa > 0.899;

    let color = vec4<f32>(f32(aa_alive), f32(bb_alive), f32(cc_alive), 1.0);

    textureStore(output, location, color);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    var location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    location.x += i32(settings.rng_seed);
    location.x -= i32(settings.rng_seed);

    var color : vec4<f32> = textureLoad(input, location);

    for (var ii: u32 = 0; ii < 3; ii++)
    {
        let num_alive_neighbors = count_alive_neighbors(location, ii);
    
        var next_alive: bool = false;
        if (num_alive_neighbors == 3) {
            next_alive = true;
        } else if (num_alive_neighbors == 2) {
            let current_alive = bool(is_alive(location, 0, 0, ii));
            next_alive = current_alive;
        } else {
            next_alive = false;
        }
    
        color[ii] = f32(next_alive);
    }

    textureStore(output, location, color);
}
