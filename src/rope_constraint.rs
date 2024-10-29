use bevy::prelude::*;
use crate::cuboid::Cuboid;
use crate::physics::PhysicsBody;

#[derive(Component)]
pub struct RopeConstraint {
    pub body_a: Entity, // 连接的第一个实体
    pub body_b: Entity, // 连接的第二个实体
    pub max_distance: f32, // 绳子最大长度
}

#[derive(Resource, Default)]
pub struct RopeConstraints {
    pub constraints: Vec<RopeConstraint>,
}

impl RopeConstraint {
    pub fn new(body_a: Entity, body_b: Entity, max_distance: f32) -> Self {
        Self {
            body_a,
            body_b,
            max_distance,
        }
    }
}

// 维持距离约束的系统
pub fn maintain_rope_constraints(
    mut constraints: ResMut<RopeConstraints>,
    mut query_set: ParamSet<(
        Query<&mut PhysicsBody>, 
        Query<&mut PhysicsBody>
    )>,
) {
    for constraint in constraints.constraints.iter() {
        // 首先获取 `body_a` 的 PhysicsBody
        let position_a;
        {
            let mut p0 = query_set.p0(); // 获取 `p0` 的查询
            let physics_a = p0.get_mut(constraint.body_a).unwrap();
            position_a = physics_a.predicted_position; // 获取位置
        } // `p0` 在这里释放，确保没有冲突

        // 然后获取 `body_b` 的 PhysicsBody
        let position_b;
        {
            let mut p1 = query_set.p1(); // 获取 `p1` 的查询
            let physics_b = p1.get_mut(constraint.body_b).unwrap();
            position_b = physics_b.predicted_position; // 获取位置
        } // `p1` 在这里释放

        // 再次获取可变引用以进行修改
        let direction = (position_b - position_a).normalize();
        let distance = position_a.distance(position_b);

        if distance > constraint.max_distance {
            // 如果距离超过最大值，则限制物体的速度
            query_set.p0().get_mut(constraint.body_a).unwrap().cancel_velocity_along(direction);
            query_set.p1().get_mut(constraint.body_b).unwrap().cancel_velocity_along(direction);
        }
    }
}


