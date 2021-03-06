extern crate cgmath;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate shrev;
extern crate specs;

use cgmath::{Point2, Rad, Rotation2, Transform};
use shrev::EventChannel;
use specs::prelude::{Builder, RunNow, World, WorldExt};

use rhusics_core::Pose;
use rhusics_ecs::collide2d::{
    BasicCollisionSystem2, BodyPose2, BroadBruteForce2, CollisionMode, CollisionShape2,
    CollisionStrategy, ContactEvent2, Rectangle, GJK2,
};

pub fn main() {
    let mut world = World::new();

    let mut system = BasicCollisionSystem2::<f32, BodyPose2<f32>, ()>::new()
        .with_broad_phase(BroadBruteForce2::default())
        .with_narrow_phase(GJK2::new());
    system.setup(&mut world);

    let mut reader_1 = world
        .write_resource::<EventChannel<ContactEvent2<f32>>>()
        .register_reader();

    world
        .create_entity()
        .with(CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(10., 10.).into(),
        )).with(BodyPose2::<f32>::one())
        .build();

    world
        .create_entity()
        .with(CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
            CollisionStrategy::FullResolution,
            CollisionMode::Discrete,
            Rectangle::new(10., 10.).into(),
        )).with(BodyPose2::<f32>::new(
            Point2::new(3., 2.),
            Rotation2::from_angle(Rad(0.)),
        )).build();

    system.run_now(&world);
    println!(
        "Contacts: {:?}",
        world
            .read_resource::<EventChannel<ContactEvent2<f32>>>()
            .read(&mut reader_1)
            .collect::<Vec<_>>()
    );
}
