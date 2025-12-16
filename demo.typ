#set page(
  header: align(
    right + horizon,
    context document.title,
  ),
  numbering: "1",
  columns: 2,
)

#set par(justify: true)
#set text(
  size: 11pt,
)
#show title: set text(size: 17pt)
#show title: set align(center)
#show title: set block(below: 1.2em)
#show heading: set align(center)
#show heading: set text(
  size: 13pt,
  weight: "regular",
)
#show heading: smallcaps

#place(
  top + center,
  float: true,
  scope: "parent",
  clearance: 2em,
)[
  #title[
    A Fluid Dynamic Model
    for Glacier Flow
  ]


  #grid(
    columns: (1fr, 1fr),
    align(center)[
      Therese Tungsten \
      Artos Institute \
      #link("mailto:tung@artos.edu")
    ],
    align(center)[
      Dr. John Doe \
      Artos Institute \
      #link("mailto:doe@artos.edu")
    ],
  )

  #align(center)[
    #set par(justify: false)
    *Abstract* \
    #lorem(80)
  ]

]

= Introduction
#lorem(200)
= Related Work
#lorem(200)
