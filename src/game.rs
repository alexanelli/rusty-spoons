use std::rc::Rc;
use yew::prelude::*;

/// reducer's Action
enum GameAction {
  Double,
  Square,
  UpgradeClicks,
  UpgradeSpeed,
  Click,
}

/// reducer's State
struct GameState {
  //Increment this by one every click
  pub clicks: i32,
  //autmoatically click this many times per second
  pub auto_clicks: i32,
  //cost of the upgrades, increases exponentially
  pub cost: i32,
  //level of the speed up upgrade
  pub upgrade_speed: i32,
  //ms between each autoclick
  pub click_rate: i32,
  //storing interval here so we can update it
  pub interval_auto: i32,
  //clicks per physical click
  pub click_increment: i32,
}

impl Default for GameState {
  fn default() -> Self {
    Self { 
      auto_clicks: 0,
      cost: 1,
      upgrade_speed: 0,
      click_rate: 1000,
      click_increment: 1,
      interval_auto: 1,
      clicks: 0
    }
  }
}

impl Reducible for GameState {
  /// Reducer Action Type
  type Action = GameAction;

  /// Reducer Function
  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      GameAction::Double =>{
        Self { clicks: self.clicks * 2, ..*self }.into()
      },
      GameAction::Square => {
        Self { clicks: self.clicks.pow(2), ..*self }.into()
      },
      GameAction::Click => {
        Self { clicks: self.clicks + 1, ..*self }.into()
      },
      GameAction::UpgradeClicks => {
        Self { click_increment: self.click_increment + 1, ..*self }.into()
      },
      GameAction::UpgradeSpeed => {
        Self { click_rate: self.click_rate + 1, ..*self }.into()
      },
    }
  }
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub name: AttrValue,
}

#[function_component]
pub fn GameBoard(props: &Props) -> Html {

  // The use_reducer hook takes an initialization function which will be called only once.
  let game = use_reducer(GameState::default);

  let double_onclick = {
    let game = game.clone();
    Callback::from(move |_| game.dispatch(GameAction::Double))
  };
  let square_onclick = {
    let game = game.clone();
    Callback::from(move |_| game.dispatch(GameAction::Square))
  };
  let click_onclick = {
    let game = game.clone();
    Callback::from(move |_| game.dispatch(GameAction::Click))
  };
  let upgrade_clicks_onclick = {
    let game = game.clone();
    Callback::from(move |_| game.dispatch(GameAction::UpgradeClicks))
  };
  let upgrade_speed_onclick = {
    let game = game.clone();
    Callback::from(move |_| game.dispatch(GameAction::UpgradeSpeed))
  };

  html! {
    <>
      <div> {props.name.clone()} </div>
      <div id="result">{ game.clicks }</div>

      <button onclick={double_onclick}>{ "Double" }</button>
      <button onclick={square_onclick}>{ "Square" }</button>
      <button onclick={click_onclick}>{ "Click" }</button>
      <div>{ "click power: "}{game.click_increment}</div>
      <button onclick={upgrade_clicks_onclick}>{ "Upgrade Clicks" }</button>
      <div>{ "auto click rate: "}{game.click_rate}</div>
      <button onclick={upgrade_speed_onclick}>{ "Upgrade Speed" }</button>
    </>
  }
}




// #[derive(Clone, Debug, PartialEq, Copy)]
// pub struct Game {
//   //Increment this by one every click
//   pub clicks: i32,
//   //autmoatically click this many times per second
//   pub auto_clicks: i32,
//   //cost of the upgrades, increases exponentially
//   pub cost: i32,
//   //level of the speed up upgrade
//   pub upgrade_speed: i32,
//   //ms between each autoclick
//   pub click_rate: i32,
//   //storing interval here so we can update it
//   pub interval_auto: i32,
//   //clicks per physical click
//   pub click_increment: i32,
// }

// impl Game {

//   pub async fn start_game_loop(&self) {
//     spawn_local(async {
//       thread::spawn(|| {
//         loop {
//           thread::sleep(Duration::from_millis(self.click_rate));
//         }
//       });
//     });
//   }

//   pub fn main_click_btn_handler(&self) {
//     self.clicks += self.click_increment;
//   }
//   fn cant_afford(&self, cost: i32) -> bool{
//     println!("you can't afford this");
//     return self.clicks < cost;
//   }

//   pub fn buy_click_btn_handler(&self) {
//     if cant_afford(self.cost) { return; }

//     self.clicks -= self.cost;
//     self.auto_clicks += 1;
//     self.cost = i32::pow(2, self.auto_clicks);

//   }

//   pub fn upgrade_speed_btn_handler(&self) {
//     let upgrade_cost = i32::pow(3, self.upgrade_speed) * 100;
//     if cant_afford(upgrade_cost){ return; }

//     self.clicks -= self.upgrade_cost;
//     self.upgrade_speed += 1;
//     self.click_rate = self.click_rate * 0.90;
//   }

//   pub fn increase_click_btn_handler(&self) {
//     let upgrade_cost = i32::pow(3, self.click_increment);
//     if cant_afford(upgrade_cost){ return; }
//     click_increment += 1;
//   }

//   // pub fn set_storage(&self){
//   // //use gloo-storage LocalStorage
//   // }

//   // pub fn get_storage(&self){

//   // }
// }

