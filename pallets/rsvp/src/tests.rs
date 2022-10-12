use crate::mock::*;
use crate::Workshops;
use frame_support::{assert_ok, pallet_prelude::*};

#[test]
fn create_workshop_works() {
	new_test_ext().execute_with(|| {
		let workshop_name = "Hackathon".encode();

		assert_ok!(Rsvp::create_workshop(RuntimeOrigin::signed(1), workshop_name.clone()));
		assert_eq!(Workshops::<Test>::iter_keys().count(), 1);

		let workshop_id = Workshops::<Test>::iter_keys().next();

		assert!(workshop_id.is_some());
		assert_eq!(
			Rsvp::workshops(workshop_id.unwrap()),
			Some((BoundedVec::truncate_from(workshop_name), 1))
		);
	});
}
