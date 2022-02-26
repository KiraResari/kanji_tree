# Legend

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

## Definition of how to determine parents of a Kanji

* A Kanji's parents **must** contain all the strokes in the Kanji
* A Kanji should have as few parents as possible
  * If there are multiple options to write a Kanji with equally few parents, then the option that combines the most complex parent with one or more simple parents should be used
* A Kanji that consists of two of the same parent should only list that parent once 
  * e.g.: 林 has just one parent: 木
  * In case of double->triple Kanji, the triple Kanji should be assigned the single and double parents if the arrangement fits
    * e.g.: 
      * 森 has the parents 木 and 林
      * But 晶 has only one parent: 日, because the double of 日 (昌 has one parent: 日) is vertical in arrangement, but 晶 has the two lower 日 (which form a unit) next to each other, and there is no kanji that features two 日 next to each other which could server as a component



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
  * 



ESSENTIAL OUTSTANDING REQUIREMENTS:

* Implement X-Parts
* Refactor `kanji.json` into sections by type (Kanji, Radical, X-Part, Dead...)



TODO:

* Renaming: "Nodes", because a lot if the things I call "Kanji" now can actually be a bunch of different things, such as Radicals



# ⚓