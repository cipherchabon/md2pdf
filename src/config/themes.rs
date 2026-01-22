pub fn get_theme_preamble(theme: &str, paper: &str) -> String {
    match theme {
        "github" => github_theme(paper),
        "academic" => academic_theme(paper),
        "minimal" => minimal_theme(paper),
        _ => default_theme(paper),
    }
}

fn default_theme(paper: &str) -> String {
    format!(
        r##"#set page(paper: "{paper}", margin: (x: 2.5cm, y: 2.5cm))
#set text(size: 11pt)
#set heading(numbering: none)
#set par(justify: true, leading: 0.65em)

#show heading.where(level: 1): it => {{
  set text(size: 20pt, weight: "bold")
  block(above: 1.5em, below: 0.8em, it)
}}

#show heading.where(level: 2): it => {{
  set text(size: 16pt, weight: "bold")
  block(above: 1.3em, below: 0.6em, it)
}}

#show heading.where(level: 3): it => {{
  set text(size: 13pt, weight: "bold")
  block(above: 1.2em, below: 0.5em, it)
}}

#show raw.where(block: true): it => {{
  set text(size: 9pt)
  block(
    fill: luma(245),
    inset: 10pt,
    radius: 4pt,
    width: 100%,
    it
  )
}}

#show raw.where(block: false): it => {{
  box(
    fill: luma(240),
    inset: (x: 3pt, y: 0pt),
    outset: (y: 3pt),
    radius: 2pt,
    it
  )
}}

#show link: it => {{
  set text(fill: rgb("#0366d6"))
  underline(it)
}}
"##
    )
}

fn github_theme(paper: &str) -> String {
    format!(
        r##"#set page(paper: "{paper}", margin: (x: 2cm, y: 2cm))
#set text(font: "Inter", size: 10.5pt)
#set heading(numbering: none)
#set par(justify: false, leading: 0.7em)

#show heading.where(level: 1): it => {{
  set text(size: 24pt, weight: "bold")
  block(above: 1.2em, below: 0.5em, {{
    it
    line(length: 100%, stroke: 0.5pt + luma(200))
  }})
}}

#show heading.where(level: 2): it => {{
  set text(size: 18pt, weight: "bold")
  block(above: 1.2em, below: 0.5em, {{
    it
    line(length: 100%, stroke: 0.5pt + luma(200))
  }})
}}

#show heading.where(level: 3): it => {{
  set text(size: 14pt, weight: "bold")
  block(above: 1em, below: 0.4em, it)
}}

#show raw.where(block: true): it => {{
  set text(font: "Fira Code", size: 9pt)
  block(
    fill: rgb("#f6f8fa"),
    inset: 12pt,
    radius: 6pt,
    width: 100%,
    it
  )
}}

#show raw.where(block: false): it => {{
  set text(font: "Fira Code", size: 9pt)
  box(
    fill: rgb("#f6f8fa"),
    inset: (x: 4pt, y: 2pt),
    radius: 3pt,
    it
  )
}}

#show link: it => {{
  set text(fill: rgb("#0366d6"))
  it
}}
"##
    )
}

fn academic_theme(paper: &str) -> String {
    format!(
        r##"#set page(paper: "{paper}", margin: (x: 3cm, y: 3cm))
#set text(size: 12pt)
#set heading(numbering: "1.1")
#set par(justify: true, leading: 0.8em, first-line-indent: 1em)

#show heading.where(level: 1): it => {{
  set text(size: 16pt, weight: "bold")
  block(above: 2em, below: 1em, it)
}}

#show heading.where(level: 2): it => {{
  set text(size: 14pt, weight: "bold")
  block(above: 1.5em, below: 0.8em, it)
}}

#show heading.where(level: 3): it => {{
  set text(size: 12pt, weight: "bold", style: "italic")
  block(above: 1.2em, below: 0.6em, it)
}}

#show raw.where(block: true): it => {{
  set text(font: "Menlo", size: 9pt)
  block(
    stroke: 0.5pt + luma(180),
    inset: 10pt,
    width: 100%,
    it
  )
}}

#show raw.where(block: false): it => {{
  set text(font: "Menlo", size: 10pt)
  it
}}

#show link: it => {{
  set text(fill: blue)
  it
}}
"##
    )
}

fn minimal_theme(paper: &str) -> String {
    format!(
        r##"#set page(paper: "{paper}", margin: (x: 2cm, y: 2cm))
#set text(font: "Inter", size: 11pt)
#set heading(numbering: none)
#set par(justify: false, leading: 0.65em)

#show heading.where(level: 1): it => {{
  set text(size: 18pt, weight: "medium")
  block(above: 1.5em, below: 0.8em, it)
}}

#show heading.where(level: 2): it => {{
  set text(size: 14pt, weight: "medium")
  block(above: 1.2em, below: 0.6em, it)
}}

#show heading.where(level: 3): it => {{
  set text(size: 12pt, weight: "medium")
  block(above: 1em, below: 0.5em, it)
}}

#show raw.where(block: true): it => {{
  set text(size: 9pt)
  block(
    inset: 10pt,
    width: 100%,
    it
  )
}}

#show link: it => {{
  underline(it)
}}
"##
    )
}
