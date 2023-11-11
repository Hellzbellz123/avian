#![allow(missing_docs)]

use crate::prelude::*;
use bevy::ecs::query::WorldQuery;
use std::ops::{AddAssign, SubAssign};

#[derive(WorldQuery)]
pub struct IsRigidBody {
    rb_2d: Has<RigidBody2d>,
    rb_3d: Has<RigidBody3d>,
}

impl<'w> IsRigidBodyItem<'w> {
    pub fn get(&self) -> bool {
        self.rb_2d || self.rb_3d
    }
}

/// A [`WorldQuery`] to make querying and modifying rigid bodies more convenient.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct RigidBodyQuery2d {
    pub entity: Entity,
    pub rb: Ref<'static, RigidBody2d>,
    pub position: &'static mut Position2d,
    pub rotation: &'static mut Rotation2d,
    pub previous_position: &'static mut PreviousPosition2d,
    pub previous_rotation: &'static mut PreviousRotation2d,
    pub accumulated_translation: &'static mut AccumulatedTranslation2d,
    pub linear_velocity: &'static mut LinearVelocity2d,
    pub(crate) pre_solve_linear_velocity: &'static mut PreSolveLinearVelocity2d,
    pub angular_velocity: &'static mut AngularVelocity2d,
    pub(crate) pre_solve_angular_velocity: &'static mut PreSolveAngularVelocity2d,
    pub mass: &'static mut Mass,
    pub inverse_mass: &'static mut InverseMass,
    pub inertia: &'static mut Inertia2d,
    pub inverse_inertia: &'static mut InverseInertia2d,
    pub center_of_mass: &'static mut CenterOfMass2d,
    pub friction: &'static Friction,
    pub restitution: &'static Restitution,
    pub locked_axes: Option<&'static LockedAxes2d>,
    pub dominance: Option<&'static Dominance>,
}

impl<'w> RigidBodyQuery2dItem<'w> {
    /// Computes the effective inverse mass, taking into account any translation locking.
    pub fn effective_inv_mass(&self) -> Vector2 {
        let mut inv_mass = Vector2::splat(self.inverse_mass.0);

        if let Some(locked_axes) = self.locked_axes {
            inv_mass = locked_axes.apply_to_vec(inv_mass);
        }

        inv_mass
    }

    /// Computes the effective world-space inverse inertia, taking into account any rotation locking.
    pub fn effective_world_inv_inertia(&self) -> Scalar {
        let mut inv_inertia = self.inverse_inertia.0;

        if let Some(locked_axes) = self.locked_axes {
            inv_inertia = locked_axes.apply_to_rotation(inv_inertia);
        }

        inv_inertia
    }

    /// Returns the current position of the body. This is a sum of the [`Position`] and
    /// [`AccumulatedTranslation`] components.
    pub fn current_position(&self) -> Vector2 {
        self.position.0 + self.accumulated_translation.0
    }

    /// Returns the [dominance](Dominance) of the body.
    ///
    /// If it isn't specified, the default of `0` is returned for dynamic bodies.
    /// For static and kinematic bodies, `i8::MAX` (`127`) is always returned instead.
    pub fn dominance(&self) -> i8 {
        if !self.rb.is_dynamic() {
            i8::MAX
        } else {
            self.dominance.map_or(0, |dominance| dominance.0)
        }
    }
}

/// A [`WorldQuery`] to make querying and modifying rigid bodies more convenient.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct RigidBodyQuery3d {
    pub entity: Entity,
    pub rb: Ref<'static, RigidBody3d>,
    pub position: &'static mut Position3d,
    pub rotation: &'static mut Rotation3d,
    pub previous_position: &'static mut PreviousPosition3d,
    pub previous_rotation: &'static mut PreviousRotation3d,
    pub accumulated_translation: &'static mut AccumulatedTranslation3d,
    pub linear_velocity: &'static mut LinearVelocity3d,
    pub(crate) pre_solve_linear_velocity: &'static mut PreSolveLinearVelocity3d,
    pub angular_velocity: &'static mut AngularVelocity3d,
    pub(crate) pre_solve_angular_velocity: &'static mut PreSolveAngularVelocity3d,
    pub mass: &'static mut Mass,
    pub inverse_mass: &'static mut InverseMass,
    pub inertia: &'static mut Inertia3d,
    pub inverse_inertia: &'static mut InverseInertia3d,
    pub center_of_mass: &'static mut CenterOfMass3d,
    pub friction: &'static Friction,
    pub restitution: &'static Restitution,
    pub locked_axes: Option<&'static LockedAxes3d>,
    pub dominance: Option<&'static Dominance>,
}

impl<'w> RigidBodyQuery3dItem<'w> {
    /// Computes the effective inverse mass, taking into account any translation locking.
    pub fn effective_inv_mass(&self) -> Vector3 {
        let mut inv_mass = Vector3::splat(self.inverse_mass.0);

        if let Some(locked_axes) = self.locked_axes {
            inv_mass = locked_axes.apply_to_vec(inv_mass);
        }

        inv_mass
    }

    /// Computes the effective world-space inverse inertia tensor, taking into account any rotation locking.
    pub fn effective_world_inv_inertia(&self) -> Matrix3 {
        let mut inv_inertia = self.inverse_inertia.rotated(&self.rotation).0;

        if let Some(locked_axes) = self.locked_axes {
            inv_inertia = locked_axes.apply_to_rotation(inv_inertia);
        }

        inv_inertia
    }

    /// Returns the current position of the body. This is a sum of the [`Position`] and
    /// [`AccumulatedTranslation`] components.
    pub fn current_position(&self) -> Vector3 {
        self.position.0 + self.accumulated_translation.0
    }

    /// Returns the [dominance](Dominance) of the body.
    ///
    /// If it isn't specified, the default of `0` is returned for dynamic bodies.
    /// For static and kinematic bodies, `i8::MAX` (`127`) is always returned instead.
    pub fn dominance(&self) -> i8 {
        if !self.rb.is_dynamic() {
            i8::MAX
        } else {
            self.dominance.map_or(0, |dominance| dominance.0)
        }
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct MassPropertiesQuery2d {
    pub mass: &'static mut Mass,
    pub inverse_mass: &'static mut InverseMass,
    pub inertia: &'static mut Inertia2d,
    pub inverse_inertia: &'static mut InverseInertia2d,
    pub center_of_mass: &'static mut CenterOfMass2d,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct MassPropertiesQuery3d {
    pub mass: &'static mut Mass,
    pub inverse_mass: &'static mut InverseMass,
    pub inertia: &'static mut Inertia3d,
    pub inverse_inertia: &'static mut InverseInertia3d,
    pub center_of_mass: &'static mut CenterOfMass3d,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct ColliderQuery2d {
    pub collider: &'static mut Collider2d,
    pub aabb: &'static mut ColliderAabb2d,
    pub mass_properties: &'static mut ColliderMassProperties2d,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct ColliderQuery3d {
    pub collider: &'static mut Collider3d,
    pub aabb: &'static mut ColliderAabb3d,
    pub mass_properties: &'static mut ColliderMassProperties3d,
}

impl<'w> AddAssign<ColliderMassProperties2d> for MassPropertiesQuery2dItem<'w> {
    fn add_assign(&mut self, rhs: ColliderMassProperties2d) {
        let new_mass = self.mass.0 + rhs.mass.0;

        if new_mass <= 0.0 {
            return;
        }

        let com1 = self.center_of_mass.0;
        let com2 = rhs.center_of_mass.0;

        // Compute the combined center of mass and combined inertia tensor
        let new_com = (com1 * self.mass.0 + com2 * rhs.mass.0) / new_mass;
        let i1 = self.inertia.shifted(self.mass.0, new_com - com1);
        let i2 = rhs.inertia.shifted(rhs.mass.0, new_com - com2);
        let new_inertia = i1 + i2;

        // Update mass properties
        self.mass.0 = new_mass;
        self.inverse_mass.0 = 1.0 / self.mass.0;
        self.inertia.0 = new_inertia;
        self.inverse_inertia.0 = self.inertia.inverse().0;
        self.center_of_mass.0 = new_com;
    }
}

impl<'w> SubAssign<ColliderMassProperties2d> for MassPropertiesQuery2dItem<'w> {
    fn sub_assign(&mut self, rhs: ColliderMassProperties2d) {
        if self.mass.0 + rhs.mass.0 <= 0.0 {
            return;
        }

        let new_mass = (self.mass.0 - rhs.mass.0).max(0.0);
        let com1 = self.center_of_mass.0;
        let com2 = rhs.center_of_mass.0;

        // Compute the combined center of mass and combined inertia tensor
        let new_com = if new_mass > Scalar::EPSILON {
            (com1 * self.mass.0 - com2 * rhs.mass.0) / new_mass
        } else {
            com1
        };
        let i1 = self.inertia.shifted(self.mass.0, new_com - com1);
        let i2 = rhs.inertia.shifted(rhs.mass.0, new_com - com2);
        let new_inertia = i1 - i2;

        // Update mass properties
        self.mass.0 = new_mass;
        self.inverse_mass.0 = 1.0 / self.mass.0;
        self.inertia.0 = new_inertia;
        self.inverse_inertia.0 = self.inertia.inverse().0;
        self.center_of_mass.0 = new_com;
    }
}

impl<'w> AddAssign<ColliderMassProperties3d> for MassPropertiesQuery3dItem<'w> {
    fn add_assign(&mut self, rhs: ColliderMassProperties3d) {
        let new_mass = self.mass.0 + rhs.mass.0;

        if new_mass <= 0.0 {
            return;
        }

        let com1 = self.center_of_mass.0;
        let com2 = rhs.center_of_mass.0;

        // Compute the combined center of mass and combined inertia tensor
        let new_com = (com1 * self.mass.0 + com2 * rhs.mass.0) / new_mass;
        let i1 = self.inertia.shifted(self.mass.0, new_com - com1);
        let i2 = rhs.inertia.shifted(rhs.mass.0, new_com - com2);
        let new_inertia = i1 + i2;

        // Update mass properties
        self.mass.0 = new_mass;
        self.inverse_mass.0 = 1.0 / self.mass.0;
        self.inertia.0 = new_inertia;
        self.inverse_inertia.0 = self.inertia.inverse().0;
        self.center_of_mass.0 = new_com;
    }
}

impl<'w> SubAssign<ColliderMassProperties3d> for MassPropertiesQuery3dItem<'w> {
    fn sub_assign(&mut self, rhs: ColliderMassProperties3d) {
        if self.mass.0 + rhs.mass.0 <= 0.0 {
            return;
        }

        let new_mass = (self.mass.0 - rhs.mass.0).max(0.0);
        let com1 = self.center_of_mass.0;
        let com2 = rhs.center_of_mass.0;

        // Compute the combined center of mass and combined inertia tensor
        let new_com = if new_mass > Scalar::EPSILON {
            (com1 * self.mass.0 - com2 * rhs.mass.0) / new_mass
        } else {
            com1
        };
        let i1 = self.inertia.shifted(self.mass.0, new_com - com1);
        let i2 = rhs.inertia.shifted(rhs.mass.0, new_com - com2);
        let new_inertia = i1 - i2;

        // Update mass properties
        self.mass.0 = new_mass;
        self.inverse_mass.0 = 1.0 / self.mass.0;
        self.inertia.0 = new_inertia;
        self.inverse_inertia.0 = self.inertia.inverse().0;
        self.center_of_mass.0 = new_com;
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use approx::assert_relative_eq;
    use bevy::prelude::*;

    // TODO: Test if inertia values are correct
    #[test]
    fn mass_properties_add_assign_works() {
        // Create app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn an entity with mass properties
        app.world.spawn(MassProperties2dBundle {
            mass: Mass(1.6),
            inverse_mass: InverseMass(1.0 / 1.6),
            center_of_mass: CenterOfMass2d(Vector2::NEG_X * 3.8),
            ..default()
        });

        // Create collider mass properties that will be added to the existing mass properties
        let collider_mass_props = ColliderMassProperties2d {
            mass: Mass(8.1),
            inverse_mass: InverseMass(1.0 / 8.1),
            center_of_mass: CenterOfMass2d(Vector2::X * 1.2 + Vector2::Y),
            ..default()
        };

        // Get the mass properties and add the collider mass properties
        let mut query = app.world.query::<MassPropertiesQuery2d>();
        let mut mass_props = query.single_mut(&mut app.world);
        mass_props += collider_mass_props;

        // Test if values are correct
        // (reference values were calculated by hand)
        assert_relative_eq!(mass_props.mass.0, 9.7);
        assert_relative_eq!(mass_props.inverse_mass.0, 1.0 / 9.7);
        assert_relative_eq!(
            mass_props.center_of_mass.0,
            Vector2::X * 0.375_257 + Vector2::Y * 0.835_051,
            epsilon = 0.000_001
        );
    }

    // TODO: Test if inertia values are correct
    #[test]
    fn mass_properties_sub_assign_works() {
        // Create app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn an entity with mass properties
        app.world.spawn(MassProperties2dBundle {
            mass: Mass(8.1),
            inverse_mass: InverseMass(1.0 / 8.1),
            center_of_mass: CenterOfMass2d(Vector2::NEG_X * 3.8),
            ..default()
        });

        // Create collider mass properties that will be subtracted from the existing mass properties
        let collider_mass_props = ColliderMassProperties2d {
            mass: Mass(1.6),
            inverse_mass: InverseMass(1.0 / 1.6),
            center_of_mass: CenterOfMass2d(Vector2::X * 1.2 + Vector2::Y),
            ..default()
        };

        // Get the mass properties and subtract the collider mass properties
        let mut query = app.world.query::<MassPropertiesQuery2d>();
        let mut mass_props = query.single_mut(&mut app.world);
        mass_props -= collider_mass_props;

        // Test if values are correct.
        // The reference values were calculated by hand.
        // The center of mass is computed as: (com1 * mass1 - com2 * mass2) / (mass1 - mass2).max(0.0)
        assert_relative_eq!(mass_props.mass.0, 6.5);
        assert_relative_eq!(mass_props.inverse_mass.0, 1.0 / 6.5);
        assert_relative_eq!(
            mass_props.center_of_mass.0,
            Vector2::NEG_X * 5.030_769 + Vector2::NEG_Y * 0.246_153,
            epsilon = 0.000_001
        );
    }

    #[test]
    fn mass_properties_add_sub_works() {
        // Create app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let original_mass_props =
            MassProperties2dBundle::new_computed(&Collider2d::capsule(2.4, 0.6), 3.9);

        // Spawn an entity with mass properties
        app.world.spawn(original_mass_props.clone());

        // Create collider mass properties
        let collider_mass_props = Collider2d::capsule(7.4, 2.1).mass_properties(14.3);

        // Get the mass properties and then add and subtract the collider mass properties
        let mut query = app.world.query::<MassPropertiesQuery2d>();
        let mut mass_props = query.single_mut(&mut app.world);
        mass_props += collider_mass_props;
        mass_props -= collider_mass_props;

        // Test if values are correct. They should be equal to the original values.
        // Some epsilons reduced to make test pass on apple-m1
        // see: https://github.com/Jondolf/bevy_xpbd/issues/137
        assert_relative_eq!(
            mass_props.mass.0,
            original_mass_props.mass.0,
            epsilon = 0.001
        );
        assert_relative_eq!(
            mass_props.inverse_mass.0,
            original_mass_props.inverse_mass.0,
            epsilon = 0.000_001
        );
        assert_relative_eq!(
            mass_props.inertia.0,
            original_mass_props.inertia.0,
            epsilon = 0.001
        );
        assert_relative_eq!(
            mass_props.inverse_inertia.0,
            original_mass_props.inverse_inertia.0,
            epsilon = 0.001
        );
        assert_relative_eq!(
            mass_props.center_of_mass.0,
            original_mass_props.center_of_mass.0,
            epsilon = 0.000_001
        );
    }
}
