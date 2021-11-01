# HULK

---
> Hulk is an **ultra simple** static site generator designed to appeal to both technical and non technical users.

> Hulk converts Markdown (*.md*) files (from data folder) into html files.
---

- You need to have cargo installed. Its installation is very simple on windows as well and linex systems.
- Secondly it is better if you have **Visual Studio Code** installed as well. It is not must but is advised. You can also use any other text editor like Atom, sublime text etc.

---

### Installation
At the moment this application is just available on cargo. To install you have to type into terminal **cargo install hulk**

> cargo install hulk

---
### Usage
- Creat an empty folder and name it any thing you want.
- Open the terminal in this folder and type **cargo run init**. This will create a **data** folder for you to write mark-down files into and a **site** folder where the static files will be copied.

> cargo run init

- Once you have created few mark-down files in data folder you can type into terminal **cargo run gen** and the static site will be generated in **site** folder.

> cargo run gen

- An index.html will also been generated.
- The names of the folders are **data** and **site** since other names like src, dist etc are mostly taken by other applications.
- At the moment Hulk just process mark-down files which has **.md** extention. All other files are ignored. 

### Special Features
- I intend to keep this tool zero configuration thus no matter how advance it gets it will always be usable out of box with zero configuration. Thats why it is very openionated. 
- However in future there will be config files etc to customize Hulk. 

### Help is Needed
You help, support, guidance and suggestions are expected and desired. 

---
**Please do report bugs and suggestions here [https://github.com/skillzaa/hulk/issues](https://github.com/skillzaa/hulk/issues)**