# Thaimeleon Palette

The Thaimeleon palette is inspired by [Rose Pine](https://rosepinetheme.com/). 

The Thaimeleon palette is made up of forty-five colors, where at least twenty-five are guaranteed to be unique. However, in practice, you probably will be using quarter of it. The palette can be split into five different "sets".

## Set 1 

Set 1 is used for primary background colors.

| Name | Description |
| :----------- | :----------- |
| base | Main background |
| highlight | Highlight within a base layer |
| surface | Secondary background for layers within the main background|

## Set 2

Set 2 is used for background layers and borders.

| Name | Description |
| :----------- | :----------- |
| overlay | Tertiary background |
| background accent 1 | Main accent background to highlighted surfaces |
| background accent 2 | Secondary accent background to highlighted surfaces  |
| background accent 3 | Tertiary accent background to highlighted surfaces |
| background accent 4 | Supplemental accent background to highlighted surfaces |
| background accent 5 | Supplemental accent background to highlighted surfaces |
| background accent 6 | Supplemental accent background to highlighted surfaces |
| background red |Specified red background |
| background yellow |Specified yellow background |
| background green |Specified green background |
| background cyan |Specified cyan background |
| background blue |Specified blue background |
| background magenta |Specified magenta background |


## Set 3

Set 3 is the middle child of the family, for colors that don't fit in class 2 or 4. It is also a good fallback color for the unfortunate situation where colors act both in foreground and background. However, please use that as a last restort.

| Name | Description |
| :----------- | :----------- |
| muted | Muted content |
| regular accent 1 | Main accent |
| regular accent 2 | Secondary accent |
| regular accent 3 | Tertiary accent |
| regular accent 4 | Supplemental accent |
| regular accent 5 | Supplemental accent |
| regular accent 6 | Supplemental accent |
| red | Specified red accent|
| yellow | Specified yellow accent|
| green | Specified green accent|
| cyan | Specified cyan accent|
| blue | Specified blue accent|
| magenta | Specified magenta accent|

## Set 4

Set 4 is used for highlighted foreground.

| Name | Description |
| :----------- | :----------- |
| subtext | Foreground for less important content|
| foreground accent 1 | Main accent foreground |
| foreground accent 2 | Secondary accent foreground |
| foreground accent 3 | Tertiary accent foreground |
| foreground accent 4 | Supplemental accent foreground |
| foreground accent 5 | Supplemental accent foreground |
| foreground accent 6 | Supplemental accent foreground |
| foreground red |Specified red foreground |
| foreground yellow |Specified yellow foreground |
| foreground green |Specified green foreground |
| foreground cyan |Specified cyan foreground |
| foreground blue |Specified blue foreground |
| foreground magenta |Specified magenta foreground |


## Set 5

Set 5 is used for primary foregrounds.

| Name | Description |
| :----------- | :----------- |
| text | Main foreground for content that is important to be read|

## Miscellaneous

The following colors are designed for logos and art.

| Name | Description |
| :----------- | :----------- |
| white | Specified white - base color in light themes, base color in dark themes |
| black | Specified black - base color in dark themes, base color in light themes |

# Guidelines for Thaimeleon Palette

In order to ensure accessibility, use table below to determine background and foreground pairings.

| Background | Foreground |
| :----------- | :----------- |
| Set 1 | Set 4, Set 5 |
| Set 2 | Set 5 |

You might be able to flip the background and foreground around, but that really depends on the wallpaper or image if that makes sense, and most of the time, I don't think you should. I plan to add more flexibility on this in the future.

## Other tips

  - To be honest, Thaimeleon isn't designed to max productivity/accessiblity. If you are having problems, set the radius baseline in the config somewhere to 0.1-0.2 and set `set_4_dps_contrast` and `set_5_dps_contast higher` about like ten higher.

- For syntax colors, here is a blog post I mostly agree with: https://tonsky.me/blog/syntax-highlighting/
  - All colors are lightness uniform in Thaimeleon. I open to suggestions for improvements. However, Thaimeleon also has background tinting which makes things more complicated.
  - Also I think bracket contrast is important since they give structure.

* Using the main accent colors will preserve the spirit of the wallpaper the best. Preserve the labeled accent colors for specific meanings like errors and warnings. 

* Be cautious of bold weights. Colors appear differently under weights. Considering using underlining instead. 

* Feel free to make exceptions when you see fit. For example, for giant text, it might be better to use a Set 3 over Set 1. Although not ideal, you might want to grab out an [OKLCH color picker](https://oklch.com/#0.8,0.1,25,100) and make colors outside the palette. Although aesthetics are cool, accessibility is still king. 

* Use my [dotfiles](https://codeberg.org/thairanaru/dotfiles) as references (or just steal them)

