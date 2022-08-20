# Legend

* Phun: pronounced "fun"; meaning: "not fun"
* PSP: PaintShopPro



# 12-Feb-2022

* Now continuing with this

* Late last year I set up a basic foundation for the Kanji Tree together with Robert

* Thus far what I have is a relatively lightweight library that reads Kanji from a `kanji.json` file and saves them in a vector, then uses that vector to look up Kanji

* Thus far, it can do three things:

  * Get a Kanji via its identifier
  * Get a Kanji's parents via the Kanji's identifier
  * Get a Kanji's children via the Kanji's identifier

* Robert came up with that data structure, and I am really happy with it thus far

  * The fact that all the Kanji exist separately from one another in a dictionary-like structure means that I won't have to worry about any tight couplings
  * The only potential issue would be if Kanji were to be moved, but since the Kanji tree is by definition a pretty static structure based on the Japanese language, which is not at all likely to change

* To formally declare the purpose of this project again here:

  * The purpose of the Kanji Tree is to have a graphical representation of which Kanji are contained in other Kanji, for example: (日&立) -> 音 -> 闇 <- 門
  * The ultimate goal would be to have this in a flowchart-like structure, but as a first goal, I would like a view that displays a Kanji, together with its parents and children
  * Clicking on a parent or child should then open the view for that Kanji

* However, an absolute must-have for this is for there to be some sort of GUI

* I have by now finished the Rust Book (https://doc.rust-lang.org/stable/book/), which covered not only basic but already fairly advanced backend topics

  * However, there was no talk in there whatsoever of frontend topics, and an initial search already made it clear that frontend is not one of Rust's strong suits
  * And yet, there seem to be at least a few potential candidates

* Anyway, first I clearly need some sort of GUI framework or crate

  * This seems like a good place to start from: https://www.libhunt.com/l/rust/topic/gui

  * Also, I asked in the rust discord chat, and this was recommended to me:

    * https://crates.io/crates/egui

    * This is the sample page for egui, and I must say, it looks really, really good

      * https://www.egui.rs/index.html

    * The documentation for egui is here:

      * https://docs.rs/egui/latest/egui/

    * One thing I will want to **keep in mind** is this:

      * Can I use `egui` with non-latin characters?

        Yes! But you need to install your own font (`.ttf` or `.otf`) using `Context::set_fonts`.

    * I now cloned the following sample project from git

      * https://github.com/emilk/eframe_template/

      * Looks like this is supposed to be used as a template for building an application on, but I already have the kanji-tree project all set up

      * I should probably try and play around with the sample project for a bit

      * First, I'll want to create a field there in the main window that displays the current kanji

      * The first thing I tried is already not so good

      * I tried creating a "kanji" variable and have it displayed in a label, but when I try that I get the following error:

        * ````
          error[E0277]: the trait bound `WidgetText: From<&mut String>` is not satisfied
             --> src\app.rs:113:16
              |
          113 |             ui.label(kanji);
              |                ^^^^^ the trait `From<&mut String>` is not implemented for `WidgetText`
              |
              = help: the following implementations were found:
                        <WidgetText as From<&String>>
                        <WidgetText as From<&str>>
                        <WidgetText as From<Arc<eframe::egui::Galley>>>
                        <WidgetText as From<String>>
                      and 2 others
              = note: required because of the requirements on the impl of `Into<WidgetText>` for `&mut String`
          ````

        * Turns out I needed to say `&*kanji` here. Still no idea as to why, but at least it works

        * Now, I have a field that displays a `☐` instead, I suppose that's where the aforementioned font support comes in

        * It says that I need to install my own font using `Context::set_fonts`, but how do I do that, and where?

        * This looks like it might help:

          * https://users.rust-lang.org/t/is-posible-egui-change-fonts-to-japanese-how/59662/4

          * Nope, that doesn't really seem to help after all...

          * Even if I just cut and paste the whole thing, I still get the following error:

            * ``````rust
              error[E0308]: mismatched types
                --> src\app.rs:41:50
                 |
              41 |         font.font_data.insert("mPlus".to_owned(),std::borrow::Cow::Borrowed(include_bytes!("../fonts/msgothic.ttc")));
                 |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `FontData`, found enum `Cow`
                 |
                 = note: expected struct `FontData`
                              found enum `Cow<'_, [u8; 9176636]>`
              ``````

          * Anyway, here's how I got it to work in the end:

            * ``````
                      let mut font = FontDefinitions::default();
                      font.font_data.insert(
                          keys::FONT_GOTHIC.to_string(),
                          FontData{
                              font: std::borrow::Cow::Borrowed(
                                  include_bytes!("../fonts/msgothic.ttc")
                              ),
                              index: 0
                          }
                      );
                      font.fonts_for_family.get_mut(&FontFamily::Monospace).unwrap().insert(0, keys::FONT_GOTHIC.to_string());
                      font.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, keys::FONT_GOTHIC.to_string());
                      _ctx.set_fonts(font);
              ``````

  * With that, I now managed to display my first Kanji in the test project

* This is as far as I'm getting today



# 16-Feb-2022

* Now continuing with this

* Last time I played around with egui for creating a rust interface, and got some barely tolerable results

* Today, I shall try to actually get started with the KanjiTree GUI here

* Presently, I am stuck on getting my main class recognize a class in another file in the same directory

  * This is particularly weird since this just happens to work in the egui sample project, but the exact same thing doesn't seem to work for me
  * I got some help from the rust chat for this one
  * Apparently, for the `main.rs` to recognize the `app.rs`, there needs to be a `pub use app::KanjiTreeApp;` directive in the `lib.rs`
  * Anyway, now this works

* It wasn't exactly straightforward after that, but eventually I still managed to get a first window that displayed a title and a kanji

* Let that be Version 0-1-0

* Before anything else, I want to play around with the layout a bit

  * Right now, the active kanji is displayed on the top left, and is rather small

  * I want it to be in the centered and big

    * The "big" thing is actually really, really important, since at the current size of the kanji, it is going to be extremely difficult to make out some of the miniscule differences of the more complex kanji

  * Time to hit the egui reference for answers

  * ...what is the egui reference?

  * I found this. Let's see if it holds the answers that I seek

    * https://docs.rs/egui/latest/egui/
    * ...regrettably, I can't find anything there

  * The `eframe_template` doesn't seem to have anything like that either

  * I have now downloaded the `eframe` repository with more samples, and I am still looking

  * I thing it may have something to do with the number displayed behind these:

    * ``````
          job.append(
              "mixing ",
              0.0,
              TextFormat {
                  font_id: FontId::proportional(20.0),
                  color: default_color,
                  ..Default::default()
              },
          );
          job.append(
              "fonts, ",
              0.0,
              TextFormat {
                  font_id: FontId::monospace(14.0),
                  color: default_color,
                  ..Default::default()
              },
          );
      ``````

  * And there's also something like this mentioned in here:

    * https://docs.rs/egui/latest/egui/struct.FontDefinitions.html

  * ...however, there's nothing like a simple "how to" for setting a font size

  * And also, I am importing my own font here, so that might make it more difficult

  * I think I found something like that here now:

    * ```
      pub struct FontId {
          /// Height in points.
          pub size: f32,
      
          /// What font family to use.
          pub family: FontFamily,
          // TODO: weight (bold), italics, …
      }
      ```

  * ...however, it would seem that this is not yet live, and I have no idea when or if this might get released, so it means that I can't use that

  * That sucks

  * I wonder if there's any other way to do font size in egui

  * Based on what I read, there really isn't

  * That's a shame. It means I can't use egui after all

* What are alternatives?

  * Iced
    * https://github.com/iced-rs/iced

  * Tuix
    * https://rustrepo.com/repo/geom3trik-tuix
  * Druid
    * https://github.com/linebender/druid
  * Azul
    * https://rust.libhunt.com/azul-alternatives
  * Orbtk
    * https://github.com/redox-os/orbtk

* Since Iced seems to be pretty popular, I'll have a look at that now

  * The documentation for iced is here:

    * https://docs.rs/iced/latest/iced/

  * Examples for iced are here:

    * https://github.com/iced-rs/iced/tree/master/examples
    * These samples include all sorts of cool stuff that might be useful, including a Pokédex which displays random Pokémon entries, complete with pictures
      * I think I'll analyze that one, since it seems to be closely related to what I want to do with my Kanji tree
      * Most notably, I'm pretty certain this one uses different font sizes (and not just pre-programmed font styles)
    * It'll probably be better to start with a smaller sample though
    * The `counter` sample seems pretty minimal

  * I am working on this now, and making progress, but I am still running into trouble

  * But I managed to push through, and now built the first working iced app, that does not display my kanji due to an incompatible font, but in a nice, big size

  * So, I suppose I have to figure out how to make fonts work in this one too

    * I think the Pokédex sample featured different fonts

    * ...or not

    * This might be helpful though:

      * https://docs.rs/iced/0.1.1/iced/widget/struct.Text.html

      * ````
        pub fn font(self, font: Font) -> Text
        [−]
        Sets the Font of the Text.
        ````

      * Incredibly, this actually worked

  * Now I got an UI that displays a kanji in an adjustable size

* This is as far as I'm getting today



# 18-Feb-2022

* Now continuing with this

* Currently, I have a program that displays one Kanji, and in an acceptable size too

* Next, I want to have it front and centered

  * The iced Pokédex sample app already does that, so I should be able to copycat that behavior from there

  * ...somehow that doesn't quite work though

  * In that app, it's written like this:

    * ````
              Row::new()
                  .spacing(20)
                  .align_items(Alignment::Center)
      ````

  * However, when I try that, I get the following error:

    * ````
      error[E0433]: failed to resolve: use of undeclared type `Alignment`
        --> src\app.rs:40:26
         |
      40 |             .align_items(Alignment::Center)
         |                          ^^^^^^^^^ use of undeclared type `Alignment`
      ````

  * And if I try to import it, I get an error too

    * In the Pokédex app, it's imported like this:

      * ````
        use iced::{
            button, futures, image, Alignment, Application, Button, Column, Command,
            Container, Element, Length, Row, Settings, Text,
        };
        ````

    * Meanwhile, I try to import it like this:

      * ````
        use iced::{Sandbox, Column, Element, Text, Font, Alignment,};
        ````

      * This is essentially identical

    * However, in my project, I get this error:

      * ````
        error[E0432]: unresolved import `iced::Alignment`
         --> src\app.rs:1:50
          |
        1 | use iced::{Sandbox, Column, Element, Text, Font, Alignment,};
          |                                                  ^^^^^^^^^ no `Alignment` in the root
        ````

    * Since I've had one error like this once before, I now checked whether the `Alignment` class was already released, and as it turns out, this, too, is an unreleased feature

  * I now tried around a little bit, and in the end, it turned out to work still differently, but now I got a centered Kanji. Yay! =^,^=

* Okay, so next, I want the Kanji Tree to start working with the data it gets from the `Kanji.json`

  * I'd do well to think of a good way to encapsulate this. After all, it might not always be a `.json` file
  * Currently, I have in the `lib.rs` a `KanjiParser` class with the following functions:
    * `parse_kanji_json`
    * `get_children`
    * `get_parents`
    * `get_element`
  * Those clearly violate the SRP
  * I think this would be a better structure:
    * `KanjiParser`
      * `parse_kanji_json`
    * `KanjiSource`
      * `get_children`
      * `get_parents`
      * `get_element`
  * I am a bit concerned with how the tests are going to run when I separate this, and a I am considerably concerned about the trouble that splitting this up into several files is going to cause, but I saved, so at worst I'll just make no progress for a while
  * As expected, the modularity of rust causes errors again
    * I now managed to figure them out and fix them with a bit of help from the Rust chat
  * With that, I now have a separate `KanjiSource` class, and all the tests are still passing too
  * Now, what still bothers me is that the `KanjiParser` class is in the `lib.rs`. I feel like it should be in its own file too
    * That naturally also poses the problem as to where to place the tests
      * I think they should be distributed between the `KanjiParser` and the `KanjiSource`
      * Let's see if I can manage to get that working
      * Amazingly, I managed to get this working without too much trouble
      * Maybe I'm finally getting the hang of this
  * Anyway, the KanjiTree still doesn't use the imported data just yet, but at the very least I now managed to clean up the data structure to a point that I'm happy with
  * And now, I managed to integrate the `KanjiSource` and `KanjiParser` in a way that the KanjiTree now loads the `kanji.json` on startup, and displays the first Kanji in that file
  * The way I implemented it now was kinda dirty though, so I still want to clean it up
  * However, this is still a good place to stop for today

* This is as far as I'm getting today



# 19-Feb-2022

* Now continuing with this

* Last time I implemented a way to display a Kanji from the `kanji.json` that was kinda dirty

* Now, first thing, I want to clean that up

  * For starters, the functionality for returning the first Kanji belongs in the `KanjiSource`

  * A good and tricky question here is what should happen if the `kanji.json` doesn't contain any Kanji

    * It would be great if this could show an error message and quit the program, but how do I do that?
    * I feel like I'm getting sidetracked here. I should probably find a solution that I know how to do now and use that
    * So for now, let's just create a special "error Kanji" that displays the error message instead

  * Now I'm having troubles with lifetimes

    * I want the `KanjiTreeApp` to hold a reference to the currently active Kanji from the `KanjiSource`

    * So I tried this:

      * ````
        pub struct KanjiTreeApp{
            active_kanji: &Kanji,
            kanji_source: KanjiSource
        }
        ````

    * But then it complains:

      * ````
        8 |     active_kanji: &Kanji,
          |                   ^ expected named lifetime parameter
        ````

    * So then I tried this:

      * ``````
        pub struct KanjiTreeApp<'a>{
            active_kanji: &'a Kanji,
            kanji_source: KanjiSource
        }
        ``````

    * I _think_ I got this to work somehow?

  * But now I have this error:

    * ````
      error[E0515]: cannot return value referencing local variable `kanji_source`
        --> src\app.rs:25:9
         |
      24 |           let active_kanji = kanji_source.get_first_element();
         |                              -------------------------------- `kanji_source` is borrowed here
      25 | /         KanjiTreeApp{
      26 | |             active_kanji,
      27 | |             kanji_source
      28 | |         }
         | |_________^ returns a value referencing data owned by the current function
      ````

  * In the end, this amounted to this being a complex logic puzzle related to the lifetimes

  * In the end, I think doing it via IDs is easier

* So, I think everything I did until now was pretty much floundering around

* I'll start again

* Last time I implemented a way to display a Kanji from the `kanji.json` that was kinda dirty

* Now, first thing, I want to clean that up

  * For starters, I only want to pass around the IDs of Kanji
    * In my case, those are called `name`
    * I did that now
  * Next, I still think the function for getting the first Kanji belongs in the `KanjiSource`
    * I think I actually managed to do it this time, though I'm not quite happy with just how rust forced me to do it
    * Because as it is now, I can see that it will turn into me having a full copy of all the `Kanji` fields in the `KanjiTreeApp` 

* Next, I'll maybe look into copying the Kanji instead, since that's effectively what I'll be doing anyway

* This is as far as I'm getting today



# 21-Feb-2022

* Now continuing with this

* I still haven't made any significant progress

* Anyway, today I wanted to try copying Kanji instead of passing around references

  * Amazingly, I managed to do this relatively straightforwardly using the `Clone` trait

* Okay, so that's a wrap

* Next, let's try implementing some actual functionality

* The ultimate goal for he next phase is that each Kanji displays buttons for its children and parents, and that by clicking on these buttons it's possible to navigate up and down the tree

* But let's be pragmatic here

* First, let's try displaying simple fields that display the children of the first Kanji

  * The first question is, how can I display a variable number of elements here

  * I've managed to get my intent down, but I still get this error:

    * ````
      error[E0382]: use of moved value: `content`                                                                                                                                    
        --> src\app.rs:69:13
         |
      55 |         let content = Column::new()
         |             ------- move occurs because `content` has type `iced_native::widget::column::Column<'_, Message, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`, which does not implement the `Copy` trait
      ...
      69 |             content.push(
         |             ^^^^^^^ value moved here, in previous iteration of loop
      ````

    * I think I recall coming across this while reading the rust book

  * However, I still won't get to fix this today

* This is as far as I'm getting today



# 23-Feb-2022

* Now continuing with this

* I am currently trying to get a list of labels displayed, one for each child Kanji

  * Currently, I am getting the following error there:

    * ````
      error[E0382]: use of moved value: `content`                                                                                                                                    
        --> src\app.rs:69:13
         |
      55 |         let content = Column::new()
         |             ------- move occurs because `content` has type `iced_native::widget::column::Column<'_, Message, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`, which does not implement the `Copy` trait
      ...
      69 |             content.push(
         |             ^^^^^^^ value moved here, in previous iteration of loop
      ````

  * I tried searching for this error, but none of the things that were suggested there seem to work

  * Maybe one of the example projects has a similar situation that I can copycat

  * Someone from the rust chat managed to help me out with this now

  * Turns out I had to make `content` mutable, _and_ re-assign it in the loop, like so:

    * ````
      		let mut content = Column::new()
        		[...]
              for child in children {
                  content = content.push(                
                  [...]
              }
      ````

  * Now the child Kanji are displayed, below each other, as if in a column, which kinda makes sense since I think we're in a column element

  * I'll try to re-arrange that now

  * I now managed to get the Children Kanji displayed in a satisfactory way

  * Let this be V0-3-0

* Next, I want to clean this up a little bit

  * Right now, the `view` method of the `app.rs` is getting a little long
  * I want to break it down into smaller components, like, a component for each display element and group or so
  * Let's see if rust just lets me do that
  * I now managed to do that, and the result already looks so much nicer and more workable than before

* So, now, the next step is going to be turning the children labels into buttons

  * For this step, I won't expect the buttons to do any logic just yet - I just want to have them displayed 

  * Even if it doesn't need to do anything just yet, a `Button` in iced still requires a `Message` that it can send, so I first have to create this

    * All the Kanji-Buttons - regardless of whether they are Children, Parents or Siblings - should effectively perform the same message: Namely loading the Kanji of the button they are on

    * So that would mean that this would be a `LoadKanjiMessage`, and that the message would have to hold the parameter of the Kanji that it represents

    * That last part seems more tricky, since the basic `iced` tutorial does not cover a use case where a message is passed a parameter, but maybe one of the sample projects has something like that

    * Bingo! The `pane_grid` sample project has something like that, so I can use that as a reference

    * However, that thing with the button state is confusing me

      * I think there needs to be a state variable for each button, but how do I do that when the number of buttons is unknown?
      * Well, again, I think the `pane_grid` sample does something like this, I just need to figure out how it does that
      * I tried a bunch of things now, but no matter what I try, the button state just trips me up
      * And the `pane_grid` sample is too messy to be any help here
      * Maybe I need to try another approach here
      * Basically, I want a state to be created for each button that is created
      * I don't know what I did differently now, but now the thing with the state seems to work

    * However, instead, I now get this weird error:

      * ````
        error[E0277]: the trait bound `iced_native::element::Element<'_, _, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>: From<&iced_native::widget::text::Text<iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>>` is not satisfied
          --> src\app.rs:64:9
           |
        64 |         Button::new(&mut State::new(), &self.build_child_text(child.clone()))
           |         ^^^^^^^^^^^ the trait `From<&iced_native::widget::text::Text<iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>>` is not implemented for `iced_native::element::Element<'_, _, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`
           |
           = help: the following implementations were found:
                     <iced_native::element::Element<'a, Message, Renderer> as From<Image>>
                     <iced_native::element::Element<'a, Message, Renderer> as From<Svg>>
                     <iced_native::element::Element<'a, Message, Renderer> as From<Viewer<'a>>>
                     <iced_native::element::Element<'a, Message, Renderer> as From<iced::Space>>
                   and 14 others
           = note: required because of the requirements on the impl of `Into<iced_native::element::Element<'_, _, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>>` for `&iced_native::widget::text::Text<iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`
        note: required by `iced_native::widget::button::Button::<'a, Message, Renderer>::new`
          --> C:\Users\Kira Recover\.cargo\registry\src\github.com-1ecc6299db9ec823\iced_native-0.4.0\src\widget\button.rs:51:5
           |
        51 | /     pub fn new<E>(state: &'a mut State, content: E) -> Self
        52 | |     where
        53 | |         E: Into<Element<'a, Message, Renderer>>,
           | |________________________________________________^
        ````

    * Or possibly also this:

      * ````
        error[E0499]: cannot borrow `*self` as mutable more than once at a time
          --> src\app.rs:47:17
           |
        41 |     fn build_children_row(&mut self) -> Row<Message> {
           |                           - let's call the lifetime of this reference `'1`
        ...
        47 |                 self.build_child_button(child)
           |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `*self` was mutably borrowed here in the previous iteration of the loop
        ...
        50 |         children_row
           |         ------------ returning this value requires that `*self` is borrowed for `'1`
        ````

      * Someone suggested to use `\#![deny(elided_lifetimes_in_paths)]` at the top of the crate root against **that**

    * I collapsed the function down to one big super-function now again, which got rid of most of these errors, but I still have this one:

      * ````
        error[E0515]: cannot return value referencing temporary value
          --> src\app.rs:59:9
           |
        48 |                     &mut State::new(), 
           |                          ------------ temporary value created here
        ...
        59 |         children_row
           |         ^^^^^^^^^^^^ returns a value referencing data owned by the current function
        ````

    * I am beginning to get an inkling of an idea, that the Buttons should be their own structs (which is silly really), but it seems like the reason why it works in the `pane_grid` sample _seems_ to be something like this

    * However, the `pane_grid` sample is clearly to convoluted for me to understand, so maybe I'll have a look at some of the other samples, to see if there's a sample that does what I'm looking for (a variable number of buttons) in a simpler way

      * The `download_progress` sample project also dynamically adds new buttons

    * I _think_ I have something figured out that should work now, but I still get this error:

      * ``````
        error[E0515]: cannot return value referencing temporary value
          --> src\app.rs:50:9
           |
        47 |                 KanjiButton::new(child).view()
           |                 ----------------------- temporary value created here
        ...
        50 |         children_row
           |         ^^^^^^^^^^^^ returns a value referencing data owned by the current function
        ``````

    * Okay, so now I've tried a whole bunch of things, chatted with the people from the rust beginner's chat for hours, tried more things, lay down on the couch a couple of times, and I think now I am slowly starting to get a grasp on the whole situation

    * The bottom line is that the iteration in it's current form is not at the right place

    * The KanjiButtons need to be created independently of their use, and then called when needed

      * What would be a good place to call them from?
      * Let me try to put them into the `KanjiTreeApp` class, which would mean that they need to be in the constructor

    * Breakthrough!

    * We won!

* Okay, so that was a handful

* And at the same time, it fully validated my "baby steps" approach here

* Aka, I _knew_ that the buttons would not be straightforward, for whatever stupid reasons

* And now, we're facing the thing that I'm _really_ worried about

* As is, the buttons are there, clickable, but nothing happens when you click them

* Next, I want the Kanji represented by the button that is clicked to be loaded as the active Kanji

  * This, once again, bears the potential for unending grief and misery
  * ...and just to prove me wrong, this one turned out to be no problem at all, and worked pretty much exactly as I had hoped it would
  * Let us call this V0-4-0

* Now, there's just one last thing to be done before I can call it V1-0-0, and that is implementing the capability to traverse back upwards the Kanji Tree

* Before I do that, however, I want to do a little bit more cleanup

* For one, it bothers me that the `app.rs` by now also holds the struct `KanjiButton` and the enum `Message`, so I want to try and get them out of there somehow

  * I now managed to do that

* So, next, I want the parent Kanji, if any, to be displayed in the form of buttons as well, and while we're at it also make them navigable

  * Basically, this is only stuff that we already did before, so this _should_ be easy, but...

  * ...given how much pain the last set of buttons caused me, I am extremely wary about hidden pitfalls and such

  * As feared, now I am running into weird borrowing errors again:

    * ```
      error[E0499]: cannot borrow `*self` as mutable more than once at a time
        --> src\app.rs:58:19
         |
      52 |       fn build_main_column(&mut self) -> Column<Message> {
         |                            - let's call the lifetime of this reference `'1`
      53 | /         Column::new()
      54 | |             .padding(20)
      55 | |             .align_items(Align::Center)
      56 | |             .push(self.build_parents_row())
         | |                   ------------------------ first mutable borrow occurs here
      57 | |             .push(Text::new( "↓".to_string()))
      58 | |             .push(self.build_active_kanji_text())
         | |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
      59 | |             .push(Text::new( "↓".to_string()))
      60 | |             .push(self.build_children_row())
         | |____________________________________________- returning this value requires that `*self` is borrowed for `'1`
      ```

    * I managed to fix this one now by redefining the `build_parents_row` and `build_children_row` buttons to use the same funstion, taking a parameter instead, but now I get this error for it:

      * ````
        error[E0623]: lifetime mismatch
          --> src\app.rs:80:9
           |
        73 |     fn build_kanji_button_row(&mut self, kanji_buttons: &Vec<KanjiButton>) -> Row<Message> {
           |                                                         -----------------     ------------
           |                                                         |
           |                                                         this parameter and the return type are declared with different lifetimes...
        ...
        80 |         children_row
           |         ^^^^^^^^^^^^ ...but data from `kanji_buttons` is returned here
        ````

      * Lifetime issues... now there's something I didn't run into yet. Let's see if I can figure that out on my own (Spoiler: No)

      * I now managed to solve this using:

        * ````
          fn build_kanji_button_row<'a>(&mut self, kanji_buttons: &'a mut Vec<KanjiButton>) -> Row<'a, Message> 
          ````

      * But then I was back at the same error again

      * In the end, after a lot of back and forth, I settled on solving this using static functions like so:

        * ```
                  Column::new()
                      .padding(20)
                      .align_items(Align::Center)
                      .push(KanjiTreeApp::build_kanji_button_row(&mut self.parent_kanji_buttons))
                      .push(Text::new( "↓".to_string()))
                      .push(KanjiTreeApp::build_active_kanji_text(self.active_kanji.clone()))
                      .push(Text::new( "↓".to_string()))
                      .push(KanjiTreeApp::build_kanji_button_row(&mut self.child_kanji_buttons))
          ```

      * That now works, and with "works" I mean I can navigate up and down the Kanji Tree

    * With that, the most basic functionality of the KanjiTree is done!

    * Let this be V1-0-0

* That is a good place to stop for today



# 24-Feb-2022

* Now continuing with this
* For starters, I want to add additional Kanji to the `kanji.json` and see how fast we hit limitations
  * Most notably, I'll very quickly have to add radicals in order to add new origin points
  * For starters, I'll try adding the Radical "｜" and the Kanji "十" and see what happens
  * Looks like this works nicely, though for now, Kanji and Radicals are indiscernible in the Kanji Tree
  * Also, a lot of "Metadata" about the Kanji is still not being displayed
  * I should work on that eventually
  * In fact, that might make for a nice next step
  * However, first, I want to add just a few more characters to the Kanji Tree
  * While doing this, I came across a bit of a dilemma:
    * Like, what are the parents of 正?
    * I initially defined 正 as 一 + 二 + 丨
    * However, then I realized that 正 is pretty much just a 止 with a 一 on top
    * So it could more easily be described as: 正 = 止 + 一
    * But which one is correct?
    * I need a definition for that which I can adhere to for this project
    * I'll come up with the definition now
      * See under Rules at the very bottom

* Anyway, I now added a bunch of Kanji to the `kanji.json`, and the Kanji Tree still works nicely

  * More notably, since the `kanji.json` is only imported at runtime, no recompilation is necessary, so this works nice and quickly
  
* Okay, so for the next part, displaying the metadata of the Kanji

  * Now, the good part is that I already nicely encapsulated the part where the active Kanji itself is being rendered, so I should be able to extend that with relative ease

  * Of course, I am still highly apprehensive about rusts quirks and fully expect it to throw me exciting new errors when I try this

  * Actually, I think I'll try to create an `KanjiPanel` object here, since I already managed to create a custom element with the `KanjiButton`, and doing so will keep my code a bit cleaner

  * I now did that, and now the Kanji Tree displays the name, type, stroke count and stroke arrangement nicely

  * The alignment is still inconsistent, but that is a cosmetic feature that I can take care of later

  * However, what still bothers me with the solution that I currently have is that I need to declare the `KanjiPanel` with a `'static` lifetime. I figure that should be avoided

    * I'll now try to restructure the code in such a way that the lifetime of the `KanjiPanel` can be correctly elided
    * I now managed to do that by passing references to the functions instead of clones

  * Looking good so far

  * Now, as a Coup de Grace for today, I want to try if I can add such things as an outline and a color to the `KanjiPanel`

  *  The `color_palette` sample project of `iced` might help with this

    * Actually, that one is kinda complicated

  * The `pane_grid` also has something like that

    * That has something like this which _may_ be what I'm looking for:

      * ````
            impl container::StyleSheet for Pane {
                fn style(&self) -> container::Style {
                    container::Style {
                        background: Some(Background::Color(SURFACE)),
                        border_width: 2.0,
                        border_color: match self {
                            Self::Active => Color::from_rgb(0.7, 0.7, 0.7),
                            Self::Focused => Color::BLACK,
                        },
                        ..Default::default()
                    }
                }
            }
        ````

  * And the `styling` sample project also seems kinda relevant for this

  * Anyway, I am making progress, but once again, the styling seems more complicated, so I'll need more time for this than I have today

* This is as far as I'm getting with this today



# 25-Feb-2022

* I don't have a lot of time today, so I don't think I'll be making any progress with what I'm stuck at, but let's try

* I am currently trying to apply a style to the `KanjiPanel`

  * To do so, I am trying to use the `container::Style` struct, and the `.style` method of the `Container`

  * That, however, results in the following error:

    * ````
      error[E0283]: type annotations needed
        --> src\kanji_panel.rs:26:11
         |
      26 |         ).style(style.into())
         |           ^^^^^ ------------ this method call resolves to `T`
         |           |
         |           cannot infer type for type parameter `impl Into<Renderer::Style>` declared on the associated function `style`
         |
         = note: cannot satisfy `_: Into<Box<(dyn iced::container::StyleSheet + 'static)>>`
      ````

  * Turns out the problem here was the `.into()`, which "falsified" the error

  * After removing the `into()`, I got the actual error, which is:

    * ````
      error[E0277]: the trait bound `&iced::container::Style: From<&iced::container::Style>` is not satisfied
        --> src\kanji_panel.rs:26:17
         |
      26 |         ).style(&style)
         |           ----- ^^^^^^ the trait `iced::container::StyleSheet` is not implemented for `&iced::container::Style`
         |           |
         |           required by a bound introduced by this call
         |
         = note: required because of the requirements on the impl of `From<&iced::container::Style>` for `Box<(dyn iced::container::StyleSheet + 'static)>`
         = note: required because of the requirements on the impl of `Into<Box<(dyn iced::container::StyleSheet + 'static)>>` for `&iced::container::Style`
      ````

  * I now posted this discussion about this:

    * https://github.com/iced-rs/iced/discussions/1266

* This is as far as I'm getting today



# 26-Feb-2022

* Now, It's judgement day for this project

* Two weeks ago I started full-on development on the Rust Kanji Tree

* Since then, I managed to put about  25 hours of work time into this, most of which were spent frantically trying to figure out things that are easy in other programming languages, _including_ Delphi

* It has not been a pleasant experience

* In Java or C#, and probably even Delphi, I would have had a fully functional and probably even nice-looking Kanji Tree by now

* In Rust, I am nowhere near as far as I wanted to be at this point

* Today, I wanted to be at a stage where I could insert the new Kanji I learnt today, but that is as of yet not possible

* _However_, that stage is also not far off

* And while programming in Rust has not been enjoyable, at the very least it was educational

* And so, I now make the choice to continue programming the Kanji Tree in Rust, but also will make a note to never again start a personal project in this language

* So, now continuing with this

* I am _still_ trying to figure out how to make a colored, box-like element using iced

  * Last time, I wrote a help request about this on the iced forum
    * https://github.com/iced-rs/iced/discussions/1266
  * And a amazingly, I even got a helpful answer
    * "Basically your style needs to implement ‘From’ for each widget you want  to put it on, then that let’s rust “find” the right value for each  widget, which you then you can implement the style sheet for each widget"
  * And there were a bunch of samples too
  * Let's see if I can figure it out with that
  * Breakthrough! We won!
  * Now I managed to have the Kanji in a nice box, but that sure wasn't easy
  * I now also added a style for radicals, and also moved the styles into the `node_Type.rs` so I can use the same enum for node types and formats specific to them 
  * And while I'm at it, let's see if I can use that same logic to color the buttons too
    * SUCCESS: This now works too
  * Let this be V1-1-0

* Next, I think I need to do something about my `kanji.json`

  * Right now, this is just a big `.json` that holds an array, which in turn may hold any kind of node

  * However, I can already tell that this is going to get chaotic, especially when different types like Radicals and XParts get added

  * So, what I'd want at this point was to adjust the `kanji.json` so that it contains all the Kanji in a `kanji` array, all the Radicals in a `radicals` array, and so on and so forth

  * That will effectively enforce that Kanji, Radicals and XParts are always grouped together in the `kanji.json` source file

  * Of course, adjusting the `kanji.json` itself is not going to be difficult

  * The difficult part is going to be adjusting the `KanjiParser` so it can handle this

  * I currently use `serde_json` for that

  * The documentation for that is here:

    * https://docs.serde.rs/serde_json/

  * Internally, the data structure can stay the same. This is purely for the sake of input

  * So, let's see, how to go about this?

  * First, I'll create a sample `.json` file that I want the `KanjiParser` to be able to parse

    * I did that now

  * Now the difficult thing is going to be writing a parser that can parse this

    * My first attempt failed like this:

      * ``````
        thread 'kanji_parser::tests::parse_kanji_json_with_separate_sections_should_not_return_error' panicked at 'invalid type: map, expected a sequence at line 1 column 0', src\kanji_parser.rs:102:27
        stack backtrace:
           0: std::panicking::begin_panic_handler
                     at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:517     
           1: core::panicking::panic_fmt
        ``````

    * I tried fixing this now, but the error still persists

    * Okay, turns out this was because I tried to serialize a `Vector` where all there was was a series of differently named objects

    * The next challenge is going to be that now all the fields are missing their node_type

    * Okay, so now I managed to get the basic parsing to pass

    * And with some more effort (and help from the rust chat), I managed to get it to work in detail too

  * Now, the next step is going to be migrating this over and hoping that everything still works

  * Of course I saved in advance

  * Looks like this worked with only minor little issues!

  * So with that, the refactoring is complete, and successful

* The next two things that need to be done are:

  * Implement X-Parts
  * Rename "Kanji"

* The implementation of the X-Parts is more essential, but also more complicated

* The renaming of the elements that I indiscriminately called "Kanji" is not as essential, but should be pretty routine

  * So I'll do that now
  * Essentially, since I started with Kanji, the name "Kanji" got extended to a lot of things, even though they are not Kanji
    * Notably, Radicals have been affected by that, and X-Parts would be next
  * Basically, what the Kanji Tree does is display Elements that can be Kanji, Radicals, X-Parts and others in a tree view
  * I need a name to collectively call those by
  * "Node" comes to mind, but then, I already noticed that the `iced` lib has a `Node` struct, so I'd rather call it something else to avoid conflicts
  * "Element" has the same issue
  * I've thought about "TreeNode", but that is kinda long
  * "Character" isn't good either, since the struct already contains a "character" field
  * How about "Glyph"?
    * Nah, that is also used by rusttype:Glyph
  * And "Symbol" has the same issue
  * Okay, how about we go about this another way?
  * We already use "Kanji", which is, technically, a Japanese word native to the problem domain
  * Why not use another Japanese word for this dilemma?
    * The pros would be that it would be clear, easily recognizable, and confusing it with names from 3rd party libs is pretty much impossible
    * The cons would be... that it might look a bit strange?
    * But then again, it'll mostly only be me working with this, and anyone else who looks at it should be able to understand the encoding intuitively since it appears literally everywhere
  * So, Japanese it is
  * And I've already decided on a name: "Kigou" (meaning "Symbol" in Japanese) 
  * Now let's see how the refactoring engine will take this
    * Remarkably well, I'd say
    * I wish the Delphi IDE worked as good as this
    * For the record, I am using VisualStudioCode with the following plugins:
      * Rust and Friends
        * Rust Flash Snippets
        * Rust Mod Generator
        * CodeLLDB
        * Syntax Highlighter
        * Bracket Pair Colorizer 2
        * Better TOML
        * crates
        * rust-analyzer
        * Rust Syntax
      * Rust Doc Viewer
      * Syntax Highlighter
      * Test Adapter Converter
      * Test Explorer UI

* Anyway, this is as far as I'm getting today



# 2-Mar-2022

* Now continuing with this

* My goal for today is to whip the KanjiTree into workable shape

* For that, I need to add two more functunalities:

  * Kana
  * X-Parts

* The Kana should be routine, since they work fundamentally the same as the Kanji and the Radicals, but the X-Parts are more problematic

* So I'll focus on the X-Parts first

  * The X-Parts have an image in place of a font character, and that image should be displayed instead

  * Now, I know for a fact that such a thing is possible, because the `Pokédex` sample app of iced does just that

  * This looks like a puzzle piece:

    * ````
      struct Pokemon {
          [...]
          image: image::Handle,
          image_viewer: image::viewer::State,
      }
      
      impl Pokemon {
          [...]
          fn view(&mut self) -> Element<Message> {
              Row::new()
                  [...]
                  .push(image::Viewer::new(
                      &mut self.image_viewer,
                      self.image.clone(),
                  ))
      ````

  * However, the way that the image is loaded looks kinda funky:

    * ````
          async fn fetch_image(id: u16) -> Result<image::Handle, reqwest::Error> {
              let url = format!(
                  "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png",
                  id
              );
      
              #[cfg(not(target_arch = "wasm32"))]
              {
                  let bytes = reqwest::get(&url).await?.bytes().await?;
      
                  Ok(image::Handle::from_memory(bytes.as_ref().to_vec()))
              }
      
              #[cfg(target_arch = "wasm32")]
              Ok(image::Handle::from_path(url))
          }
      ````

  * Perhaps I should look for an easier way...

  * The `svg` sample project draws an `.svg` graphic like this:

    * ````
              let svg = Svg::from_path(format!(
                  "{}/resources/tiger.svg",
                  env!("CARGO_MANIFEST_DIR")
              ))
              .width(Length::Fill)
              .height(Length::Fill);
            
              Container::new(svg)
                  .width(Length::Fill)
                  .height(Length::Fill)
                  .padding(20)
                  .center_x()
                  .center_y()
                  .into()
      ````

    * So maybe drawing an image that is located on the disk works in a similar way

  * The `tour` sample project also contains pictures

    * ````
       [...]
       .push(Image::new("tour/images/ferris.png"))
       [...]
      ````

    * Well, that looks easy

  * Anyway, I now have some ideas on how this should work, so let's see if we can get it to work

  * First of all, I need to add a new field to the `Kigou` that can take an image name

    * I think I'm really just going to use the name, and build the path when calling it

  * For that, I figure it makes sense to first add a sample file that uses an X Part, and then write a test for it to see if the import works

    * Naturally, it fails at first, but that's what I expected

    * Now I got that to work

    * Now, the next part is going to be actually displaying the images

    * Aaand, of course that's super complicated

    * I am currently failing trying to create a function that returns either an image or a text, because I get an error like this:

      * ````
          --> src\kigou_panel.rs:48:9
           |
        48 |         Container::new(
           |         ^^^^^^^^^^^^^^ the trait `iced_graphics::backend::Image` is not implemented for `iced_wgpu::backend::Backend`
           |
           = note: required because of the requirements on the impl of `iced_native::widget::image::Renderer` for `iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>`     
           = note: required because of the requirements on the impl of `From<Image>` for `iced_native::element::Element<'_, _, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`
           = note: required because of the requirements on the impl of `Into<iced_native::element::Element<'_, _, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>>` for `Image`
        ````

    * My original attempt was something like this:

      * ````
            fn build_kigou_display(kigou: &Kigou) -> Element<Message>{
                if(kigou.uses_image()){
                    KigouPanel::build_kigou_image(kigou)
                }else{
                    KigouPanel::build_kigou_character(kigou.character.clone()).into()
                }
            }
        ````

    * But that fails with the following message:

      * ````
        31 |         if(kigou.uses_image()){
        32 |             KigouPanel::build_kigou_image(kigou)
           |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `iced_native::element::Element`, found struct `Image`
           |
           = note: expected struct `iced_native::element::Element<'_, Message, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`
                      found struct `Image`
        ````

    * The problem is that I don't know what the common supertype of Text and Image is

    * Well, in the sample projects, the images are always in a container, like this:

      * ````
        fn ferris<'a>(width: u16) -> Container<'a, StepMessage> {
            Container::new(
                // This should go away once we unify resource loading on native
                // platforms
                if cfg!(target_arch = "wasm32") {
                    Image::new("tour/images/ferris.png")
                } else {
                    Image::new(format!(
                        "{}/images/ferris.png",
                        env!("CARGO_MANIFEST_DIR")
                    ))
                }
                .width(Length::Units(width)),
            )
            .width(Length::Fill)
            .center_x()
        }
        ````

    * However, even if I just copy that, I still get this error again right away:

      * ````
        error[E0277]: the trait bound `iced_wgpu::backend::Backend: iced_graphics::backend::Image` is not satisfied
          --> src\kigou_panel.rs:48:9
           |
        48 |         Container::new(
           |         ^^^^^^^^^^^^^^ the trait `iced_graphics::backend::Image` is not implemented for `iced_wgpu::backend::Backend`
           |
           = note: required because of the requirements on the impl of `iced_native::widget::image::Renderer` for `iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>`     
           = note: required because of the requirements on the impl of `From<Image>` for `iced_native::element::Element<'_, _, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`
           = note: required because of the requirements on the impl of `Into<iced_native::element::Element<'_, _, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>>` for `Image`
        note: required by `iced_native::widget::container::Container::<'a, Message, Renderer>::new`
        ````

    * So, what in Lerra?

    * So, this might help:

      * https://github.com/iced-rs/iced/issues/258

    * Looks like this is a dependency issue

    * Phun

    * I mean, it clearly works in the `iced` sample project...

    * Okay, so after a bunch of trying, I've now gotten it to work by writing the sections "features" and "dependencies" in the `Cargo.toml` like this:

      * ````
        [features]
        default = ["wgpu"]
        # Enables the `iced_wgpu` renderer
        wgpu = ["iced_wgpu"]
        # Enables the `Image` widget
        image = ["iced_wgpu/image"]
        
        [dependencies]
        serde_json = "1.0"
        serde = { version = "1.0", features = ["derive"] }
        iced = {version = "0.3", features = ["image"] }
        iced_wgpu = { version = "0.4", optional = true }
        ````

    * "Work" as in "it compiles". I still need to go and see if the images are actually displayed

    * Aand, it actually displays the image now, but much to big yet

    * Also, the color of the buttons is still wrong

    * And the image is still missing in the buttons

    * But that it even works in the first place is already the fuck amazing!

    * Okay, so now the colors work too, and the size of the X Part looks acceptable

    * Next, I need to get the X Parts working in the `KigouButton` too

    * And that now works too

    * Maybe the sizes and scaling are still a bit bogus, but for now, this will do

  * Next, I noticed that the logic I created for drawing the Kigou on the `KigouButton` and `KigouPanel` are very similar, so I'll try to draw it out now

    * I now did that, and it looks good
    * Yay! Happy!

  * With that, the X Parts are officially working now

* So now I just have to add the Kana

  * That should be easy
  * And indeed, it was

* Sooo, now all the basic elements work

* The Kanji, the Radicals, the X Parts and the Kana are all functional, and I can add new ones by adding them in the `kanji.json`

* With that, the primary objective of the Kanji Tree is now fulfilled: I can now continue to add new Kanji as I learn them

* Of course, there's still plenty to be done in this project, but at the very least, this is now an n-complete version that I can both leave as it is, and also use as a solid starting point for further improvements

* So, let this be Version 2-0-0

* For the rest of today, I'll try to add Kanji and see if I run into any problems doing that



# 9-Mar-2022

* So, I've now worked with what I have for a week, and found that it generally works

* However, there are a few functionalities that I'm really missing, so I'll try to implement these today

* Those are, in order of importance:

  * Dead Kanji
  * Reload Button
  * Search
  * Executable
  * Validations on import

* I'll try to take care of as many of those as possible today

* Starting with the Dead Kanji

  * Those are pretty straightforward, since they are pretty much just another color-scheme of Kigou
  * I'll have to adjust the .json import for that again, however
  * Definition of Dead:
    * A Kanji that is not used in any words
    * Example: 斿
      * This is used in 遊, but doesn't appear in any words on its own
  * That now works

* Next, the reload button

  * The use case for this is:

    * When I make changes to the `kanji.json`, I want to be able to press a key or button to reload the `kanji.json` so that I can see my changes without having to restart the entire program

  * Basically, I think that means I'll have to re-create the `KigouSource` and reload the view

  * To do that, I think I need to add another type of `Message`

  * It all worked fine until I tried adding the button to the form, at which point I got this error:

    * ````
      error[E0515]: cannot return value referencing temporary value
        --> src\app.rs:54:9
         |
      54 | /         Column::new()
      55 | |             .padding(20)
      56 | |             .push(ReloadButton::new().view())
         | |                   ------------------- temporary value created here
      57 | |             .align_items(Align::Center)
      ...  |
      61 | |             .push(Text::new( "↓".to_string()))
      62 | |             .push(KanjiTreeApp::build_kanji_button_row(&mut self.child_kigou_buttons))
         | |______________________________________________________________________________________^ returns a value referencing data owned by the current function
      ````

    * I more or less understand the error, but I don't know what to do about it

    * After some thought, I managed to fix this by creating a `reload_button: ReloadButton,`-field in the `KanjiTreeApp`, populating that during the startup, and then just using that value here

    * That fixed this issue

  * Now the Reload button exists and seems to work, but I still need to put it to the test

  * SUCCESS: That now works as intended!

  * Now, it would be great if I could figure out how to add a hotkey for that too, but I wasn't able to find out how, and I can't recall seeing anything like that in any of the examples, so since this is not essential, and I have no idea how to do it, I'll just let it be for now

  * However, it would also be great if the app could stay on the current Kigou (if it still exists) instead of skipping back to the beginning on a reload

    * And here, I now _do_ have a pretty good idea of how to achieve that, so I'll do that now

  * SUCCESS: That now works too!

  * With that, I think I'm satisfied enough with the Reload functionality that I can move on to the next issue

* The Search

  * I want to be have a search box that I can use to jump to any Kigou, either by character or name

    * If no such Kigou exists, a message should be displayed
    * The characters should be searched fist, and then the name
    * The name search should first do an absolute search, and then a fuzzy search
      * If the fuzzy search returns multiple results, it's okay for a random one of those to be displayed

  * So, from the way I defined these requirements, I think there should be a text input field, a button, as well as a display field for the message

  * It probably makes sense for the message field to be a separate component so I can use it for other things I want to display too

  * But one step at a time

  * First, I'll implement the basic search functionality for characters

    * For that, I'll definitely need a new type of  `Message`

    * Now, how do you do a text entry field?

      * Fortunately, there's plenty of examples for this

      * I think this is the logic I'm searching for:

        * ````
                          let input = TextInput::new(
                              input,
                              "What needs to be done?",
                              input_value,
                              Message::InputChanged,
                          )
                          .padding(15)
                          .size(30)
                          .on_submit(Message::CreateTask);
          ````

      * That may mean that I might need another `Message` type, but let's see

    * Okay, so searching by character works now, but the font is not yet displaying Kanji

      * I now got the font for the Kanji to work, but the font for the "🔍" is yet still eluding me
      * Segoe UI Symbol should work for that

    * Now the fonts in the search bar look good too

  * Next, I'll implement the exact name search

    * Okay, that works now too

  * Next, I'll implement the fuzzy name search

    * That now works too!

  * And finally, I want to implement a display message

    * I managed to get this to work too!

  * With that, this part is now complete too

* With the new message bar, the app now is starting to look really, really cool

* At this point, the only two improvements on my immediate wishlist are "not displaying the arrows if now children/parents are present" and the validations on import

* The validations are probably the more important things, but since those are probably more complex (since I need to decide on which behavior I want the app to display in case of an error), and I have only a little time left today, I'll instead do the arrows thing, where I already have a pretty good idea of how it might work

* So, now trying the display arrows

  * Basically, I want the app to onl< display the arrow above the Kigou if the Kigou has parents, and only display it below it it has children
  * My initial attempt failed on account of Rust being a borrowing-nazi again
  * I now got it to work, but I'm not fully happy with it, since the borrowing rules of rust force me to keep several things in one function that I would rather have in separate functions
  * The main problem being that it won't let me borrow `self` multiple times again, even though that seemingly already happens, I really don't know why
  * Anyway, it works for now, and it doesn't look _too_ bad, so I'll leave it at that for now

* This is as far as I'm getting today

* So, I've done quite a few things today, and the Kanji Tree looks that much better for it

* I think this is deserving of a new version number

* So, let this be Version 2-1-0



# 10-Mar-2022

* Now continuing with this

* Today, I want to try and work with the produced .`exe` file and deployment in general for a bit

  * For one, it would be great if the deployment worked in such a way that the .`exe` would work where it is deployed to

    * For that, I think I need to clean up the project file structure a bit
    * But first, let's see what is possible
    * Right now, what ends up in the `\target\release` folder is the `kanji_tree.exe`, which crashes when run there because it can't find the `kanji.json`
    * However, when I copy that  `kanji_tree.exe` into the project's base directory, where the `kanji.json` is located, it works just fine
    * I've been looking, but wasn't able to find a straightforward way for this
    * However, as bottom line, I should probably gather everything I need into the `resources` folder
    * I now did that

  * Now, let's have a look at how to give the `exe` an icon

    * This looks promising:

      * https://stackoverflow.com/questions/30291757/attaching-an-icon-resource-to-a-rust-application

    * Success: Now the `.exe` bears the Kanji Tree R Icon

    * However, the app still misses this while running

    * Very fortunately, the thread also has an iced-specific answer to that problem:

      * https://docs.rs/iced/latest/iced/window/struct.Settings.html#structfield.icon

      * However, that doesn't actually tell me how to do that

      * I now tried this, but that doesn't seem to work:

        * ````
          pub fn main() -> iced::Result {
              let mut settings = Settings::default();
              settings.window.icon =  Icon::from_rgba(
                  include_bytes!(
                      "../resources/images/Kanji Tree R Icon.ico"
                  ).to_vec(),
                  512,
                  512
              ).ok();
              kanji_tree::KanjiTreeApp::run(settings)
          }
          ````

      * And the iced samples don't seem to feature app icons either

      * Debugging this, I now found that the `from_rgba` function returns this error:

        * ````
          DimensionsMismatch
          * width: 512
          * height: 512
          * pixel_count: 34359
          ````

      * If I change the "512"s above to "32"s, the error changes to:

        * ````
          DimensionsMismatch
          * width: 32
          * height: 32
          * pixel_count: 34359
          ````

      * 34359 is not a value that amounts to a squared image though...

      * But just out of interest, I'll try making the icon smaller and see what happens

      * That now changes the error to:

        * ````
          InvalidData
          * byte_count: 4286
          * [raw]
          ** variant0:
          *** byte_count: 4286
          ** variant1:
          *** width: 385
          *** height: 4286
          *** pixel_count: 1653562408960
          ````

      * Okay, that makes even less sense

      * Maybe I should try giving it a .png instead?

      * That returns a DimensionMismatch again

        * ````
          DimensionsMismatch
          * width: 32
          * height: 32
          * pixel_count: 49700
          ````

      * Last try: Bitmap 32x32

      * That returns the InvalidData error again, this time with:

        * width: 463
        * height: 3126
        * pixel_count: 1988569858048

    * Seriously, WTF?

    * Okay, so I'm obviously not getting this to run on my own. Time to ask for help

    * I have now created this help request about this:

      * https://github.com/iced-rs/iced/discussions/1281

    * In the meantime, I'll just leave this in since it's not breaking anything

  * I also remembered that I still need to put the resources in the font folder

    * And while I'm at it, I should probably also unify the font access somehow, because right now, it's in several places all over the code
    * I now did that, and the code looks so much nicer for it

* I think this is as far as I'm getting development-wise for today

* ...I'm genuinely disappointed that the `exe` is 19.2 MB big though. That's approximately 19 MB more than I would have expected

  * I looked into that, and it would seem that it's the msgothic font files that make it so big
  * I suppose I can't do without those if I want to use Kanji
  * I also did a quick compare with the Arial Unicode, and that one is even bigger, featuring a whopping 22MB, so not an option

* For the rest of today, I simply added a few more Kanji



# 26-Mar-2022

* Now continuing with this

* While adding more Kanji, I noticed that the font that I'm using can't display the Kigou ⺮

  * That's a kinda critical issue, so even though I don't have any development time now, I am still interrupting my other projects to take care of that

  * I'm currently using the MS Gothic font to display Kigou

  * That font has two files, but I am currently only including one of them, so that may be the issue

  * I don't know how to include both files in the same font object, however

  * But let's first make sure that the font can even display this character

  * I tried writing this character with MS Gothic in PSP, and it worked

    * Notably, writing it in Arial Unicode there _didn't_ work

  * Let's see if I can figure out how to add the extra file to the font

  * Fonts require a byte array, so maybe I can just add up two byte arrays?

  * This doesn't seem to work:

    * ````
              let complete_bytes 
                  = [
                      include_bytes!("../resources/fonts/msgothic.ttc"),
                      include_bytes!("../resources/fonts/msgothic_0.ttc")
                  ].concat();
      ````

    * It gives this error:

      * ````
        12 |                 include_bytes!("../resources/fonts/msgothic_0.ttc")
           |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 9176636 elements, found one with 9165480 elements
        ````

  * Maybe this will help?

    * https://docs.rs/byte-strings/0.1.3/byte_strings/macro.concat_bytes.html
    * Nope, that doesn't seem to work

  * After a LONG session of trial and error, I finally got it to work like this:

    * `cargo.toml`

    * ```
      [dependencies]
      [...]
      array-concat = "0.5.1"
      ```

    * ```
      use array_concat::*;
      
      const MS_GOTHIC_PART_1: [u8; 9176636] = *include_bytes!("../resources/fonts/msgothic.ttc");
      const MS_GOTHIC_PART_2: [u8; 9165480] = *include_bytes!("../resources/fonts/msgothic_0.ttc");
      const MS_GOTHIC: [u8; concat_arrays_size!(MS_GOTHIC_PART_1, MS_GOTHIC_PART_2)]
       = concat_arrays!(MS_GOTHIC_PART_1, MS_GOTHIC_PART_2);
      ```

  * HOWEVER, the Kigou ⺮ is still not displayed

  * That sucks

  * THAT SUCKS!!!

  * I now wrote an issue for that here:

    * https://github.com/iced-rs/iced/issues/1295

  * Anyway, change of plans

  * Maybe I need a different font

  * When trying to ascertain if the font supported that character, I noticed that Microsoft Word always replaced the font without asking, so maybe I can use the font it replaced it to?

  * That font is SimSun

    * That is also a multi-file font, but let's try
    * Okay, but that seems to work! Yay!
    * The Kigou are "serif"-styled now, which is not what I'd prefer, but oh well
    * I now found another font "Noto Sans Mono CJK JP Regular" that displays the Kigou the way I prefer them, and can also display ⺮

  * Also, a nice side effect of this is that I now know how to load the fonts as constants, and cleaned up the `Fonts` class accordingly

* This is as far as I'm getting today

* Let this be V2-2-0



# 10-Aug-2022

* Now continuing with this

* I've got a list of improvements I want to implement

* Most important of those is the support for a multi-line display for children, because especially with the "一"-Kigou, there's already so many children that the display breaks

  * Now, the easiest way would be if there was an implicit line break feature in `Row`

  * Doesn't look like it

  * So I have to implement something like this myself

  * Which means I'll have to hard-code the maximum number of Kigou to display

  * On default resolution, the window can display about 30, though the boxes are sometimes of varying width, and default resolution is a little wider than half the screen

    * So I think a sensible count would be around 20 a row

  * Now, how do I do this, considering that this is rust an all?

  * I need to think about this carefully

  * Basically, what I _want_ is for the function to create a column, then iterate through all the child kigou and if the [index of current child mod 20]=0 add a new row and add it to the column, then add kigou to it

    * I am reasonably sure that this won't work in rust
    * Can I abstract it in some other way?
    * I *can* determine the number of rows that I need from the beginning through `ceil(kigou_count/20)` (or whatever the rust equivalent is)
    * Then, I can put all those rows in a vector
    * Then, I can iterate over all the kigou by index, and add them to the row at index `kigou_index div 20` 

  * Trying to implement this, I am running into the weirdest problems right away

    * First, I simply tried simply packaging the row in a column, but that caused the following error:

      * ````
        error[E0382]: use of moved value: `kigou_button_column`                                                                                                                        
           --> src\app.rs:101:9
            |
        93  |         let kigou_button_column: Column<'a, Message> = Column::new().padding(20);
            |             ------------------- move occurs because `kigou_button_column` has type `iced_native::widget::column::Column<'_, Message, iced_graphics::renderer::Renderer<iced_wgpu::backend::Backend>>`, which does not implement the `Copy` trait
        ...
        100 |         kigou_button_column.push(kigou_button_row);
            |                             ---------------------- `kigou_button_column` moved due to this method call
        101 |         kigou_button_column
            |         ^^^^^^^^^^^^^^^^^^^ value used here after move
            |
        note: this function takes ownership of the receiver `self`, which moves `kigou_button_column`
           --> C:\Users\Kira Recover\.cargo\registry\src\github.com-1ecc6299db9ec823\iced_native-0.4.0\src\widget\column.rs:95:24
            |
        95  |     pub fn push<E>(mut self, child: E) -> Self
            |                        ^^^^
        ````

    * I now managed to get that to run by changing this:

      * ````
        kigou_button_column.push(kigou_button_row);
        kigou_button_column
        ````

    * ...to this:

      * ````
        kigou_button_column.push(kigou_button_row);
        ````

    * That now results in the same result as before, but I have no idea if it will work for multiple rows

      * Nope, doesn't

    * What's weird is that this exact same logic seems to work just fine for rows

    * Are they not symmetric?

    * Strangely, just turning it around does not cause this issue

    * Then what's the problem here?

    * Problem followed problem here, and frankly, I don't have any fun at all doing this

    * However, with some inspiration from the rust community, I was now able to figure out a working solution

    * Now it basically works, but is still in need of some cleanup

    * I now did that

  * With that, this task is now complete

* That took pretty long, so I'm afraid that this is already as far as I'm getting with this today



# 11-Aug-2022

* Now continuing with this
* The next feature I want to add is that the Kanji Tree no longer crashes when an invalid `kanji.json` is loaded
  * Instead, an error message should be displayed, detailing where the problem is
  * I tried writing a test for that in the `kigou_parser`, but then realized that this doesn't make much sense because the unwrapping logic there is located in the test utility function
  * But anyway, thanks to that I was able to notice that the `kigou_parser` behaves correctly and returns a result, which may be an error
  * So, where is the unwrapping operation that causes this to fail in actual use?
  * Okay, so it would appear that this is in `app.rs` under `new()`, which doesn't really make it testable...
    * Anyway, I figure I can just manually test this one, even if that's not as nice
  * Also, another occurrence is at `reload_kigou_source`
  * I was now able to implement this relatively straightforwardly with surprisingly few problems, and none of them really held me up or slowed me down
* With that, this feature is now implemented



# 15-Aug-2022

* Now continuing with this

* Next, I want to make the Kigou name copy-able, because that is something I miss a *lot* while adding new kanji

  * The easiest way would be to make the name field mark- and editable, but I wonder if there's a way that would allow me to copy the name straight to the clipboard?

  *  Maybe this will work?

    * https://docs.rs/druid/latest/druid/struct.Clipboard.html

    * I tried importing that, but I got this error:

      * ````
           Compiling anyhow v1.0.61
           Compiling druid-shell v0.7.0
           Compiling druid v0.7.0
           Compiling kanji_tree v2.4.0 (E:\projects\rust\kanji_tree)
        error[E0432]: unresolved import `druid_shell`
         --> src\app.rs:2:5
          |
        2 | use druid_shell::{Application, Clipboard};
          |     ^^^^^^^^^^^ use of undeclared crate or module `druid_shell`
        ````

    * I now managed to get it to work using `druid::{Application, Clipboard};` instead

    * However, now I have another of those nasty lifetime issues when trying to create a button

      * ````
                let kigou_button_row: Row<'a, Message> = Row::new()
                    .push(KigouPanel::from(active_kigou))
                    .push(
                        CopyButton::new().view());
        ````

      * This gives me the error:

        * ````
          error[E0716]: temporary value dropped while borrowed
             --> src\app.rs:122:17
              |
          118 |     fn build_kigou_panel_row<'a>(active_kigou: &'a Kigou) -> Row<'a, Message> {
              |                              -- lifetime `'a` defined here
          119 |         let kigou_button_row: Row<'a, Message> = Row::new()
              |                               ---------------- type annotation requires that borrow lasts for `'a`
          ...
          122 |                 CopyButton::new().view());
              |                 ^^^^^^^^^^^^^^^^^        - temporary value is freed at the end of this statement  
              |                 |
              |                 creates a temporary which is freed while still in use
          ````

      * I tried a couple of things to fix this, but none of them worked

      * But then why does it work for the `KigouButton`?

      * Hmm, I think I put those into class variables, so maybe I have to do that for this button too?

      * Okay, so I think this works now

  * I now managed to get rid of all the compile errors and run the whole thing

  * ...however now the Kanji Tree R panics on startup with a weird error from Druid:

    * ````
      thread 'main' panicked at 'Main thread assertion failed 1 != 0', C:\Users\Kira Recover\.cargo\registry\src\github.com-1ecc6299db9ec823\druid-shell-0.7.0\src\util.rs:39:9
      stack backtrace:
         0: std::panicking::begin_panic_handler
                   at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:517
         1: std::panicking::begin_panic_fmt
                   at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c\/library\std\src\panicking.rs:460
         2: druid_shell::util::assert_main_thread
                   at C:\Users\Kira Recover\.cargo\registry\src\github.com-1ecc6299db9ec823\druid-shell-0.7.0\src\util.rs:39        
         3: druid_shell::application::Application::try_global
                   at C:\Users\Kira Recover\.cargo\registry\src\github.com-1ecc6299db9ec823\druid-shell-0.7.0\src\application.rs:127
         4: druid_shell::application::Application::global
                   at C:\Users\Kira Recover\.cargo\registry\src\github.com-1ecc6299db9ec823\druid-shell-0.7.0\src\application.rs:112
      ````

  * This could be a synergy thing between Druid and iced...

    * I tried a fix that I could think of, but it didn't work

  * So, let us just assume for the moment that Druid doesn't work with iced, and pursue different approaches

  * Based on a suggestion from the rust beginner's chat, I tried out `arboard` instead

    * That worked like a charm!

  * With that, the clipboard functionality now works!

  * Not sure if I need it, but as an added bonus, I want to check if I can also somehow assign a `Ctrl`+`C` key combination for copying

    * I now tried finding something for some time now, but without any success, and there wasn't a sample for this in the sample projects either, so yeah, probably not then

* Just out of curiosity, I am going to try and update to the latest iced version now and see what happens

  * Yeah, no, that causes the following errors:

    * ````
      error[E0432]: unresolved import `iced::Align`
       --> src\app.rs:1:68
        |
      1 | use iced::{Sandbox, Column, Element, Text, Container, Length, Row, Align};
        |                                                                    ^^^^^ no `Align` in the root
      
      error[E0432]: unresolved import `iced::Align`
       --> src\kigou_panel.rs:1:26
        |
      1 | use iced::{Text, Column, Align, Row, Container};
        |                          ^^^^^ no `Align` in the root
      
      error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
        --> src\kigou_button.rs:17:13
         |
      15 |     pub fn view(&mut self) -> Button<Message> {
         |                 --------- this data with an anonymous lifetime `'_`...
      16 |         Button::new(
      17 |             &mut self.state, 
         |             ^^^^^^^^^^^^^^^ ...is captured here...
      ...
      20 |         .style(self.kigou.clone().kigou_type)
         |          ----- ...and is required to live as long as `'static` here
      ````

  * I'll give it a quick try if I can resolve these, otherwise I'll return to the old versions again

  * Well, I did manage to get rid of the Align.issues with relative ease, which leaves just the lifetime issues again...

    * ````
      error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
        --> src\kigou_button.rs:17:13
         |
      15 |     pub fn view(&mut self) -> Button<Message> {
         |                 --------- this data with an anonymous lifetime `'_`...
      16 |         Button::new(
      17 |             &mut self.state, 
         |             ^^^^^^^^^^^^^^^ ...is captured here...
      ...
      20 |         .style(self.kigou.clone().kigou_type)
         |          ----- ...and is required to live as long as `'static` here
      
      error[E0621]: explicit lifetime required in the type of `kigou`
        --> src\kigou_panel.rs:23:11
         |
      23 |         ).style(kigou.clone().kigou_type)
         |           ^^^^^ lifetime `'static` required
      
      ````

  * Looks like something in the underlying structures from which I inherit has changed, which now requires me to adjust my structures in a way that I don't know, and which has the potential to cascade all through the program in a horrible, horrible avalanche that might tear everything to pieces

  * Yeah, no, I don't think I want to deal with that, so I'll revert

* Instead, I'll now focus on implementing more features with what I have

* For relaxation, let's add the version number

  * I now did this, and while I was unable to get it displayed at the bottom right like I wanted, at the very least it works now

* Next, I think I want to work on the validations while importing

  * This should be something testable again
  * What I want here is that if the kanji.json has some sort of non-structural content problem (like two Kigou with the same name), that `kigou_parser.parse_kanji_json` returns an `Err` that is then handled by the error handling which I already put in place to catch parsing errors
  * By that reasoning, I think the tests for this should go into the `KigouParser`, even if the validation logic will probably end up in the `KigouSource`
  * The first validation that I want to implement is checking for duplicate names
    * That took a bit, and in the end, the validation logic ended up in the `KigouParser`, but now it works
    * **SUCCESS:** I now managed to write a validator that detects duplicate names and displays an error just as intended
  * The next validation that I want to write is for Kigou with dead parents
    * That is to say, I want to validate that all the parents referenced by a Kigou really exist
    * I now managed to do that, and was able to ascertain that it works in an amazing life-example, because the validation successfully detected a pair of dead parents in my `kanji.json` right away
    * With that, this validation is now working too
  * The final validation that I have on my list is the "image could not be found", which goes for X-Parts
    * Now this one might be a bit more interesting, but let's see...
    * Now I managed to do that too
  * With that, all the validation logic is implemented

* This is as far as I'm getting with this today

* Let this be version 2-5-0



# 17-Aug-2022

* Now continuing with this

* Next, I want to  work on the capability to search for only specific types of Kigou

  * My most common use case is that I want to quickly get the name of the Water-Radical, but my current search algorithm finds the Water Kanji instead
  * My original idea was to use toggle-buttons to limit searches for certain types of Kigou, but thinking about it and looking at how the code is structured (plus the Rust factor), I think it will be easier to just add additional search buttons for each Kigou type
  * That will satisfy my use case too
  * Now, how to do that?
  * Currently, we have a `SearchPanel` that holds a button firing a `SearchForKigou` event holding the search string)
    * That is then caught by the `App`, which will iterate through a series of three functions, first searching by character, then by exact name, and then by fuzzy name
    * These will in turn call respective functions from the `KigouSource`
  * I think the best way to do that is to add an `Option<&KigouType>` parameter to `Message.SearchForKigou`
    * If that `Option` is `None`, then the current logic is performed, while if it is a `Some<&KigouType>`, then the search is only executed for that `KigouType`
  * There is _probably_ a really clever way to do that without having to iterate over all the types, I just have to figure out how to do it
  * And naturally, first I have to try and see if this works like I want it to at all in the first place
  * First, to add the  `Option<&KigouType>` to  `Message.SearchForKigou`
    * I now did that and followed through
    * I had to use `Option<KigouType>` instead though because of lifetimes, so I hope that works out later
    * Anyway, it looks like it works for now
  * The next thing that I think would make sense is to adjust the `KigouSource` so that the Kigou search function accept the  `Option<KigouType>`
    * This time around, it ended up being a `&Option<KigouType>`, but it looks like it works for now
  * Now to adjust the `KigouSource` search functions so that they actually use the `KigouType`
    * First, I am going to write failing tests for that
      * I now have two tests that fail as expected
    * Next to make them pass
      * It took a bit of input from the rust community, but in the end, this worked out reasonably well thanks to the fact that the `Kigou` themselves  have a `kigou_type` field that I was able to compare  against
    * Now the search functions work with the `KigouType` as expected
  * The next step will be adding the buttons to the `SearhPanel` and making them dispatch the `SearchForKigou` message with the correct parameters
    * I was able to do that without any problems
  * Now this works beautifully!
  * Now just out of curiosity I want to check if I can also add tooltips to those buttons
    * I think there was a sample project for that in the iced repository...
    * Okay, so it looks needlessly convoluted, so let's not do that
  * With that, this feature is now also implemented

* One thing that should be relatively simple is to place the copy button next to the Kigou name

  * Or maybe even better, I can make the Kigou Name field into a button that has that function to begin with!

  * Turns out that this is more difficult than expected, since I have to turn the `KigouPanel`, which turned out to be a static class, into a prober instantiable class so I can give it a state so I can give it a button

  * And now it creates more horrible, horrible errors

    * ````
      error[E0515]: cannot return value referencing temporary value
        --> src\kigou_panel.rs:44:9
         |
      44 | /         Column::new()
      45 | |             .padding(20)
      46 | |             .align_items(Align::Center)
      47 | |             .push(
      ...  |
      55 | |             .push(KigouDisplayBuilder::build_kigou_display(&kigou.clone(), 64))
         | |                                                             ------------- temporary value created here
      56 | |             .push(Text::new(kigou.clone().kigou_type.to_string()))
         | |__________________________________________________________________^ returns a value referencing data owned by the current function
      ````

    * I now managed to fix that by giving up on separating this into separate functions, and cramming it all into one function

    * But at least now it works

  * Now that we have this logic in place, I figure we can also use it to copy other stuff, like the Kigou itself (if it is an unicode character) 

    * Actually, that's probably going to be a bit more complicated (again), but let's start from the top
    * First, I'll change the `Message::CopyActiveKigouName` to be a message that copies the string that we pass it
    * Incidentally, I notice now that with my changes today, the performance of the Kanji Tree R has dropped notably, most notably with the "One" Kanji, that now takes several seconds to load
      * Thankfully, those performance lags only occur when launching the app from Visual Studio Code - the compile app runs just fine
    * Anyway, now we can copy the Kigou characters too

* I think that's as far as I'm getting with this today

* Let this be Version 2-6-0



# 20-Aug-2022

* Now continuing with this
* Next, I want to add a list of the last few Kigou viewed
  * Since we already established 20 as the number of Kigou that can be displayed in one row, I figure it makes sense to apply that number here too
  * I was now able to make this work with relative ease
* Next, I want to try displaying the parents in the order in which they appear in the `kanji.json`
  * I had to replace Robert's sleek iterator logic with a for-loop there, but now it works
* Finally, I want to display the children sorted by stroke count
  * I wonder if there's a clever way to do that...
  * I think this here answers my question:
    * https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
  * If I understand that logic correctly, I can also use it to sort them by type
  * Looks like this worked, and they are now sorted by count
  * I already expected this, but looking at the result, I figure what I really want after all is them to be sorted primarily by type and then by stroke count
  * Fortunately, as I said before, if I understand the logic correctly, I should be able to do that too



# Wanted Features

* Tooltip when hovering over Kigou Buttons
* Search also displays non-primary results
* Sort children by stroke count



# Rules for adding Kanji

## How to determine parents of a Kanji

* A Kanji's parents **must** contain all the strokes in the Kanji
* A Kanji should have as few parents as possible
  * If there are multiple options to write a Kanji with equally few parents, then the option that combines the most complex parent with one or more simple parents should be used
    * e.g.: 夫 = 大 + 一 and not 人 + 二
    * An exception to this rule may be made when two similar-looking Kanji (that should be siblings) would be split apart through that
      * e.g.: 章 and 草 look a lot alike, so they should be siblings; however, following above rule, 章 would have to be written as 音+十 instead of 早+立, while 草 is written as  早+艹; in this case, 章 should be written as 早+立 so that it remains a sibling to 草
* A Kanji that consists of two of the same parent should only list that parent once 
  * e.g.: 林 has just one parent: 木
  * In case of double->triple Kanji, the triple Kanji should be assigned the single and double parents if the arrangement fits
    * e.g.: 
      * 森 has the parents 木 and 林
      * But 晶 has only one parent: 日, because the double of 日 (昌 has one parent: 日) is vertical in arrangement, but 晶 has the two lower 日 (which form a unit) next to each other, and there is no kanji that features two 日 next to each other which could server as a component

## Determining the name of a Kanji

* If you know for a fact that the Kanji goes by a certain name, and that if you say that name people are going to think of that Kanji and not another, use that name
* If two Kanji go by the same name, pull up https://jisho.org and search for <name>#kanji and give the name to the Kanji that comes up first, while trying to find another name for the other Kanji where that one comes up first:
  * e.g.:
    * 試: Test
    * 験: Verification



# ⚓