# Changelog

## 0.8.0

### Enhancements

* `Input::validate_with` can take a `FnMut` (allowing multiple references)

### Breaking

* `Input::interact*` methods take `&mut self` instead of `&self`

## 0.7.0

### Enhancements

* Added `wait_for_newline` to `Confirm`
* More secure password prompt
* More documentation
* Added `interact_text` method for input prompt
* Added `inline_selections` to `ColorfulTheme`

### Breaking

* Removed `theme::CustomPromptCharacterTheme`
* `Input` validators now take the input type `T` as arg
* `Confirm` has no `default` value by default now

## 0.6.2

### Enhancements

* Updating some docs

## 0.6.1

### Bugfixes

* `theme::ColorfulTheme` default styles are for stderr

## 0.6.0

### Breaking

* Removed `theme::SelectionStyle` enum
* Allowed more customization for `theme::Theme` trait by changing methods
* Allowed more customization for `theme::ColorfulTheme` by changing members
* Renamed prompt `Confirmation` to `Confirm`
* Renamed prompt `PasswordInput` to `Password`
* Renamed prompt `OrderList` to `Sort`
* Renamed prompt `Checkboxes` to `MultiSelect`

### Enhancements

* Improved colored theme
* Improved cursor visibility manipulation
