use std::{fmt::Display, mem::transmute};

use leptos::*;
use leptos_router::*;
use uiua::ops::Primitive;

use crate::{code::*, editor::*};

#[component]
pub fn DocsHome(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Documentation"</h2>
        <h2>"Tutorial"</h2>
        <ul>
            <p>"These are meant to be read in order:"</p>
            <li><A href="basic">"Basic Stack Operations and Formatting"</A></li>
            <li><A href="math">"Math and Comparison"</A></li>
            <li><A href="arrays">"Arrays"</A></li>
        </ul>
    }
}

#[component]
pub fn Tutorial(cx: Scope, page: TutorialPage) -> impl IntoView {
    view! { cx,
        <div>
            {page.view(cx)}
        </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TutorialPage {
    Basic,
    Math,
    Arrays,
}

impl TutorialPage {
    pub const ALL: [Self; 3] = [Self::Basic, Self::Math, Self::Arrays];
    pub fn prev(&self) -> Option<Self> {
        match self {
            Self::Basic => None,
            _ => Some(unsafe { transmute(*self as u8 - 1) }),
        }
    }
    pub fn next(&self) -> Option<Self> {
        Self::ALL.get(*self as usize + 1).copied()
    }
    pub fn view(&self, cx: Scope) -> View {
        match self {
            Self::Basic => view!(cx, <TutorialBasic/>).into_view(cx),
            Self::Math => view!(cx, <TutorialMath/>).into_view(cx),
            Self::Arrays => view!(cx, <TutorialArrays/>).into_view(cx),
        }
    }
}

#[component]
pub fn TutorialBasic(cx: Scope) -> impl IntoView {
    let primitive_table: Vec<_> = Primitive::ALL
        .into_iter()
        .filter_map(|p| {
            if let (Some(name), Some(ascii), Some(_)) = (p.ident(), p.ascii(), p.unicode()) {
                Some(view! { cx,
                    <tr>
                        <td><code>{ name }</code></td>
                        <td><code>{ ascii.to_string() }</code></td>
                        <td><PrimCode prim=p/></td>
                    </tr>
                })
            } else {
                None
            }
        })
        .collect();

    view! { cx,
        <div>
            <h2>"Basic Stack Operations and Formatting"</h2>
            <h3>"The Stack"</h3>
            <hr/>
            <p>"In Uiua, all operations operate on a global stack. Each line is evaluated from right to left. A number simply pushes its value onto the stack:"</p>
            <Editor examples={&["5", "1 2 3"]} help={&["", "Click the arrows to see more examples"]}/>
            <p>"Operators pop values off the stack and push their results."</p>
            <Editor examples={&["3", "2 3", "×2 3", "1 ×2 3", "+1 ×2 3"]}/>
            <p>"In the editor, items that end up on the "<i>"top"</i>" of the stack are shown at the "<i>"bottom"</i>" of the output. This is so consecutive lines of code show their outputs in the correct order:"</p>
            <Editor examples={&["5\n+1 2\n\"Hello, World!\"\n+1 'a'"]}/>
            <br/>
            <h3>"Formatting"</h3>
            <hr/>
            <p>"Most Uiua primitives use special unicode characters. To type multiplication and division signs, you can use "<code>"*"</code>" and "<code>"%"</code>" respectively. Then, run the code to format."</p>
            <Editor examples={&["# Click Run to format!\n%6 *3 8"]} help={&["", "⇡Click⇡"]}/>
            <p>"Most primitives have names you can type rather than symbols. Formatting works on these too. "<b>"This is the primary way of entering Uiua's glyphs."</b></p>
            <Editor examples={&["max sqrt 10 mod 10 pow 2 8", "* `1 `2"]}/>
            <p>"You don't have to type the whole name, just enough to disambiguate it from others"</p>
            <Editor examples={&["(cei ceil ceili ceilin ceiling)"]}/>
            <p>"On this site, you can also click the ↧ symbol on any editor to show a pallete of all the Uiua glyphs. You can then click on any glyph to insert it into the editor."</p>
            <p>"Here is a table of all the glyphs that are typed with ASCII characters that get converted to glyphs:"</p>
            <table>
                <tr>
                    <th>"Name"</th>
                    <th>"ASCII"</th>
                    <th>"Glyph"</th>
                </tr>
                {primitive_table}
                <tr>
                    <td>"negative number"</td>
                    <td><code>"`"</code></td>
                    <td><code>"¯"</code></td>
                </tr>
            </table>
            <p>"As noted in the table, negative number literals are typed with the "<code>"`"</code>" character. This is because "<code>"-"</code>" is used for subtraction."</p>
            <Editor examples={&["+`1 `2"]}/>
        </div>
    }
}

fn maybe_code<T: Display>(cx: Scope, val: Option<T>) -> impl IntoView {
    if let Some(val) = val {
        view! { cx, <code>{ val.to_string() }</code> }.into_view(cx)
    } else {
        view! { cx, "" }.into_view(cx)
    }
}

fn primitive_rows(cx: Scope, prims: impl IntoIterator<Item = Primitive>) -> Vec<impl IntoView> {
    prims
        .into_iter()
        .map(|p| {
            let name = p.ident();
            let glyph = p.unicode();
            let ascii = p
                .ascii()
                .map(|s| s.to_string())
                .or_else(|| glyph.filter(|c| c.is_ascii()).map(|c| c.to_string()));
            let args = p.args();
            view! { cx,
                <tr>
                    <td>{maybe_code(cx, name)}</td>
                    <td>{maybe_code(cx, ascii)}</td>
                    <td><PrimCode prim=p/></td>
                    <td>{maybe_code(cx, args)}</td>
                </tr>
            }
        })
        .collect()
}

#[component]
pub fn TutorialMath(cx: Scope) -> impl IntoView {
    use Primitive::*;
    let math_table = primitive_rows(
        cx,
        [
            Add, Sub, Mul, Div, Mod, Pow, Neg, Abs, Ceil, Floor, Round, Sqrt, Sign,
        ],
    );
    let comp_table = primitive_rows(cx, [Eq, Ne, Lt, Gt, Le, Ge, Min, Max, Floor, Ceil, Round]);

    view! { cx,
        <div>
            <h2>"Math and Comparison"</h2>
            <p>"Uiua supports all the basic math operations:"</p>
            <table>
                <tr>
                    <th>"Name"</th>
                    <th>"ASCII"</th>
                    <th>"Glyph"</th>
                    <th>"Arguments"</th>
                </tr>
                {math_table}
            </table>
            <p>"Uiua also supports comparison, min/max, and rounding operations:"</p>
            <table>
                <tr>
                    <th>"Name"</th>
                    <th>"ASCII"</th>
                    <th>"Glyph"</th>
                    <th>"Arguments"</th>
                </tr>
                {comp_table}
            </table>
            <p>"Most of these are used mostly how you might think:"</p>
            <Editor examples={&["+2 5", "↥2 5", "ⁿ2 5", "⌈2.5", "√4"]}/>
            <p>"One thing to note is that non-commutative operators work backwards:"</p>
            <Editor examples={&["-2 5", "<2 5", "÷2 5"]}/>
            <p>"Uiua has no boolean types. Comparison operators return "<code>0</code>" for false and "<code>1</code>" for true:"</p>
            <Editor examples={&["=2 5", "=2 2"]}/>
            <p>"Because of how stack operations work, you can delay operations until after all the arguments are on the stack:"</p>
            <Editor examples={&["++1 2 3", "×××2 3 4 5"]}/>
            <p>"This is not special syntax. All the numbers are pushed to the stack, then the operators work on them."</p>
            <p>"Remember that you can type the names of operators and then run to format them:"</p>
            <Editor examples={&["# Click Run to format!\nmax sqrt 2 mod 10 abs `31"]}/>
        </div>
    }
}

#[component]
pub fn TutorialArrays(cx: Scope) -> impl IntoView {
    use Primitive::*;
    view! { cx,
        <div>
            <h2>"Arrays"</h2>
            <p>"Uiua is, first and foremost, an array language. The only composite data type is the multimensional array. Arrays have a lot of nice properties, and the primitive oeprations of the language are designed to make it easy to work with them. If you've only ever programmed in non-array languages, then this will be a completely foreign paradigm. In most array languages, most data structure and indeed control flow are replaced with operations on arrays."</p>
            <h3>"How do you make an array?"</h3>
            <p>"Other than through functions, Uiua has two ways to create arrays. They are called "<i>"strand notation"</i>" and "<i>"stack notation"</i>"."</p>
            <p><b>"Strand notation"</b>" uses underscores to connect elements:"</p>
            <Editor examples={&["1_2_3","\"Hello\"_\"World\"",  "0_π_⍉_5_(+1)"]}/>
            <p>"Strand notation is good when you want to create short and/or simple arrays. For longer or more complex arrays, you can use stack notation."</p>
            <p><b>"Stack notation"</b>" uses brackets to group elements:"</p>
            <Editor examples={&["[1 2 3]", "[¯5 'a' 0 \"wow\"]"]}/>
            <p>"What's cool about stack notation is that it is "<i>"not"</i>" just a way to list elements. The code between the brackets runs from right to left as it normally would. When it is done, any items on the stack higher than when it started are put into the array. This gives you some cool ways to create arrays:"</p>
            <Editor examples={&["[...5]", "[×2.×2.×2.×2 .2]", "[+1 2 +3 4]"]}/>
            <p>"Of course, you can also use stack notation to make multideimensional arrays:"</p>
            <Editor examples={&["[[1 2 3] [4 5 6]]", "[...[1 2 3]]"]}/>
            <p>"Other than their data, arrays also have a property called their "<b>"shape"</b>". Shape is a list of non-negative integers that describes the array's size along each of its axis."</p>
            <p>"We can get the array's shape with the "<PrimCode prim=Shape name=true/>" primitive. It's a triangle because a triangle is a shape."</p>
            <Editor examples={&["△[1 2 3]", "△5", "△[[1 2 3] [4 5 6]]", "△[...[1 2 3]]"]}/>
            <p>"From shape we can derive two closely-related properties called "<b>"length"</b>" and "<b>"rank"</b>"."</p>
            <p><PrimCode prim=Len name=true/>" is the number of "<i>"major cells"</i>" of the array. This is the number of elements for a 1D array and the number of rows for a 2D array."</p>
            <p><PrimCode prim=Rank name=true/>" is the number of dimensions of the array. It is defined as the length of the shape."</p>
            <Editor examples={&["△[1 2 3]\n⇀[1 2 3]\n⸫[1 2 3]"]}/>
            <p>"When creating multidimensional arrays, stack notation applies a step called "<i>"normalization"</i>". If all the items pushed to the stack have the same shape, they will combine into an array with a higher rank. If they have different shapes, then they combine into a rank 1 nested array."</p>
            <Editor examples={&["[[1 2] [3 4]]", "[[1 2] [3 4 5]]"]}/>
        </div>
    }
}
