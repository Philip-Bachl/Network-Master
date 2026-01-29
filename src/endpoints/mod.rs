pub mod gebaeude;
pub mod raum;

//TODO: Error Handling
//TODO: enable foreign keys on deletion:
//       sqlite does not handle deletion of referenced tables well
//      it requires foreign keys to be disabled even when no orphans would be left
//      currently the plan is to disable foreign keys when deleting. This would need to change going forward. (when switching dbms)
