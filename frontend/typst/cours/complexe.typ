#import "@preview/theorion:0.4.0": *
#import cosmos.fancy: *
#set text(lang: "fr")

// Define a color palette
#let orange = rgb("#FF8C00")
#let light-orange = rgb("#FFA500")
#let dark-red = rgb("#8B0000")
#let yellow = rgb("#FFD700")

// Style for the document title
#let title-style = title => {
  set align(center)
  block(
    fill: light-orange.lighten(70%),
    stroke: orange,
    inset: 10pt,
    radius: 5pt,
    width: 100%,
    align(
      center,
      text(weight: "bold", size: 1.6em, fill: dark-red, title)
    )
  )
  // Add some space after the title
  v(2.5em)
}

// Style for level 1 headings (I, II, III...)
#show heading.where(level: 1): it => {
  set text(weight: "bold", fill: dark-red, size: 1.2em)
  // Add some space before the heading
  v(1em)
  it
}

// Style for level 2 headings (1), 2), 3)...)
#show heading.where(level: 2): it => {
  set text(fill: orange)
  underline(stroke: orange + 1pt, offset: 4pt)[#it]
}

// Style for level 3 headings (a), b), c)...)
#show heading.where(level: 3): it => {
  set text(fill: dark-red)
  it
}

// Custom numbering for headings
#set heading(numbering: (..numbers) => {
  let level = numbers.len()
  if level == 1 {
    // Roman numerals for level 1
    numbering("I", numbers.at(0))
  } else if level == 2 {
    // Arabic numerals for level 2
    numbering("1)", numbers.at(1))
  } else if level == 3 {
    // Alphabetic for level 3
    numbering("a)", numbers.at(2))
  } else {
    none
  }
})

#set document(title: "Nombres complexes")

#show math.equation: it => {
  if target() == "html" {
    box(html.frame(it))
  } else {
    it
  }
}
= Rappels 
== Définition 
L'ensemble $CC$ des nombres complexes est ${a + i b | (a,b) in RR^2}$. On définit sur $CC$ deux lois $+_CC$ et $times_CC$ par 
$ forall ((a,b),(a prime, b prime)) in (RR^2)^2 (a+i b)+_CC (a prime + i b prime) = (a+a prime) + i(b+b prime) $
$ forall ((a,b),(a prime, b prime)) in (RR^2)^2 (a+i )times_CC (a prime + i b prime) = (a+a prime) + i(b+b prime) $

== Propriétés 
#enum()[$forall ((a,b), (a prime, b prime)) in (RR^2)^2 ((a+i b = a prime + i b prime) #sym.arrow.double.r.l (a=a prime "et" b = b prime )) $][$forall z in CC exists excl (a,b) in RR^2 z = a + i b$]

== Imaginaires purs: 

#definition[On appelle (nombre) imaginaire pur tout nombres complexe de la forme $i b$ où $b in RR$.]
L'ensemble des nombres imaginaires purs est noté $i RR  ({i b | b in RR})$

== Parties réelle et imaginaire 
== Conjugaison 
=== Definition 
=== Propriétés 

