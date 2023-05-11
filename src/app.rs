use yew::prelude::*;
use gloo_console::log;
use std::thread;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;


#[derive(Clone, Debug, PartialEq)]
struct Game {
  //Increment this by one every click
  clicks: i32,
  //autmoatically click this many times per second
  auto_clicks: i32,
  //cost of the upgrades, increases exponentially
  cost: i32,
  //level of the speed up upgrade
  upgrade_speed: i32,
  //ms between each autoclick
  click_rate: i32,
  //storing interval here so we can update it
  interval_auto: i32,
  //clicks per physical click
  click_increment: i32,
}

fn build_game() -> Game {
  Game {
    clicks: 0,
    auto_clicks: 0,
    cost: 1,
    upgrade_speed: 0,
    click_rate: 1000,
    click_increment: 1,
    interval_auto: 1,
  }
}

#[function_component(App)]
pub fn app() -> Html {

  let counter = use_state(|| 0);
  let onclick = {
      let counter = counter.clone();
      Callback::from(move |_| counter.set(*counter + 1))
  };

  html! {
      <div>
          <button {onclick}>{ "Increment value" }</button>
          <p>
              <b>{ "Current value: " }</b>
              { *counter }
          </p>
      </div>
  }

  // let app_header: Html = html! {
  //   <header>
  //     <h1>{ "Hello World!" }</h1>
  //     <span class="subtitle">{ "from Yew with ❤️" }</span>
  //   </header>
  // };

  // html! {
  //   <main>
  //     {app_header}
  //     <div class="grid">
  //       <div> {"atoms"} </div>
  //       <div> {"some other stuff"} </div>
  //       <div> <button onclick={Callback::from(|_| atom_buy_button())}> { "Buy" } </button> </div>
  //     </div>
  //     <hr/>


  //     <h1> {"Incremental"}</h1>
  //     <button id="click" type="button" class="btn btn-primary btn-lg btn-block">{"CLICK ME"}</button>
  //     <hr/>
  //     <p>
  //     {"Clicks:"} <span id="total_clicks"> {0} </span> <br/>
  //     {"Clicks per"} <small id="time_period"> {1.00} </small> {"second:"} <span id="clicks_per_second"> {0} </span>
  //     </p>
  //     <h3>{"Buy"}</h3>
  //     <table class="table">
  //         <tr>
  //           <td>{ "Autoclicker" }</td>
  //           <td><span id="autoclicker_level">{ "lvl 0" }</span></td>
  //           <td><button id="buy_click" type="button" class="btn btn-success">{ "Buy for 1" }</button></td>
  //           <div> <button onclick={Callback::from(|_| atom_buy_button())}> { "Buy" } </button> </div>
  //         </tr>
  //         <tr>
  //           <td>{ "Speed up by 10%" }</td>
  //           <td><span id="speed_level">{ "lvl 0" }</span></td>
  //           <td><button id="upgrade_speed" type="button" class="btn btn-success">{ "Buy for 100" }</button></td>
  //           <div> <button onclick={Callback::from(|_| atom_buy_button())}> { "Buy" } </button> </div>
  //         </tr>
  //         <tr>
  //           <td>{ "Increase Clicks Increment" }</td>
  //           <td></td>
  //           <td><button id="increase_clicks" type="button" class="btn btn-success">{ "Buy for 100" }</button></td>
  //           <div> <button onclick={Callback::from(|_| atom_buy_button())}> { "Buy" } </button> </div>
  //         </tr>
  //     </table>
  //   </main>
  // }

}

fn atom_buy_button() {
  log!("clicked atom buy button :shrug:")
}

// pub fn thing_factory(name: String, button_handler: Fn()) -> Html {
//   html! {
//     <div>
//       <h1>{name}</h1>
//       <button onclick={Callback::from(|_| button_handler)}>
//           { "Click me!" }
//       </button>
//       <hr/>
//     </div>
//   }
// }


impl Game {

  // pub async fn start_game_loop(&self) {
  //   spawn_local(async {
  //     thread::spawn(|| {
  //       loop {
  //         thread::sleep(Duration::from_millis(self.click_rate.try_into().unwrap()));
  //       }
  //     });
  //   });
  // }

  pub fn main_click_btn_handler(&mut self) {
    self.clicks += self.click_increment;
  }
  fn cant_afford(&self, cost: i32) -> bool{
    println!("you can't afford this");
    self.clicks < cost
  }

  pub fn buy_click_btn_handler(&mut self) {
    if self.cant_afford(self.cost) { return; }

    self.clicks -= self.cost;
    self.auto_clicks += 1;
    self.cost = i32::pow(2, self.auto_clicks.try_into().unwrap());

  }

  pub fn upgrade_speed_btn_handler(&mut self) {
    let upgrade_cost = i32::pow(3, self.upgrade_speed.try_into().unwrap()) * 100;
    if self.cant_afford(upgrade_cost){ return; }

    self.clicks -= upgrade_cost;
    self.upgrade_speed += 1;
    //TODO is this gross? this must be yucky lol
    self.click_rate = ((self.click_rate as f64) * 0.9) as i32;

  }

  pub fn increase_click_btn_handler(&mut self) {
    let upgrade_cost = i32::pow(3, self.click_increment.try_into().unwrap());
    if self.cant_afford(upgrade_cost){ return; }
    self.click_increment += 1;
  }

  // pub fn set_storage(&self){
  // //use gloo-storage LocalStorage
  // }

  // pub fn get_storage(&self){
  // //use gloo-storage LocalStorage
  // }
}

