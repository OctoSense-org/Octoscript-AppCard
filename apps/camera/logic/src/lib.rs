//! The camera app without a toolkit: `session` is the state machine and UX
//! rules of the Mate 70 Air camera, `scene` renders a state into absolute
//! nodes and compiles them to an Octoscript UI L0 card, `icons` holds the
//! SVG dictionary. The Makepad host (`../native`) and the OpenHarmony host
//! (`../oh`) both mount what `scene` produces.
pub mod icons;
pub mod scene;
pub mod session;
