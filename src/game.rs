use std::rc::Rc;
use yew::prelude::*;

/// reducer's Action
enum CounterAction {
  Double,
  Square,
}

/// reducer's State
struct CounterState {
  counter: i32,
}

impl Default for CounterState {
  fn default() -> Self {
    Self { counter: 1 }
  }
}

impl Reducible for CounterState {
  /// Reducer Action Type
  type Action = CounterAction;

  /// Reducer Function
  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    let next_ctr = match action {
      CounterAction::Double => self.counter * 2,
      CounterAction::Square => self.counter.pow(2),
    };

    Self { counter: next_ctr }.into()
  }
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub name: AttrValue,
}

#[function_component]
pub fn GameBoard(props: &Props) -> Html {

  // The use_reducer hook takes an initialization function which will be called only once.
  let counter = use_reducer(CounterState::default);

  let double_onclick = {
      let counter = counter.clone();
      Callback::from(move |_| counter.dispatch(CounterAction::Double))
  };
  let square_onclick = {
      let counter = counter.clone();
      Callback::from(move |_| counter.dispatch(CounterAction::Square))
  };

  html! {
      <>
          <div> {props.name.clone()} </div>
          <div id="result">{ counter.counter }</div>

          <button onclick={double_onclick}>{ "Double" }</button>
          <button onclick={square_onclick}>{ "Square" }</button>
      </>
  }
}




#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Game {
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

