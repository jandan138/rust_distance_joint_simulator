pub struct Physics;

impl Physics {
    pub fn apply_gravity(object: &mut Cuboid) {
        let gravity = Vec3::new(0.0, -9.81, 0.0); // 模拟重力
        object.update(gravity, 0.016); // 假设帧时间为1/60秒
    }
}
