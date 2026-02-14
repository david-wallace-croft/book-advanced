use super::super::menu_resource::MenuResource;
use ::bevy::prelude::*;
use ::bevy::state::state::FreelyMutableState;

#[expect(unused_mut)]
#[expect(unused_variables)]
pub(crate) fn exit<T>(mut commands: Commands) {
  println!("loading_menu::exit()");
  // TODO
}

pub(crate) fn run<T>(
  mut state: ResMut<NextState<T>>,
  menu_info: Res<MenuResource<T>>,
) where
  T: FreelyMutableState + FromWorld + States,
{
  state.set(menu_info.menu_state.clone());
}
