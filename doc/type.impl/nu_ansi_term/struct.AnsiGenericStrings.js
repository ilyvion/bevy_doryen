(function() {var type_impls = {
"nu_ansi_term":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-AnsiGenericStrings%3C'a,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nu_ansi_term/display.rs.html#122\">source</a><a href=\"#impl-PartialEq-for-AnsiGenericStrings%3C'a,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"nu_ansi_term/struct.AnsiGenericStrings.html\" title=\"struct nu_ansi_term::AnsiGenericStrings\">AnsiGenericStrings</a>&lt;'a, S&gt;<div class=\"where\">where\n    &lt;S as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html#associatedtype.Owned\" title=\"type alloc::borrow::ToOwned::Owned\">Owned</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/nu_ansi_term/display.rs.html#122\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"nu_ansi_term/struct.AnsiGenericStrings.html\" title=\"struct nu_ansi_term::AnsiGenericStrings\">AnsiGenericStrings</a>&lt;'a, S&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.76.0/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.76.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","nu_ansi_term::display::AnsiStrings","nu_ansi_term::display::AnsiByteStrings"],["<section id=\"impl-StructuralPartialEq-for-AnsiGenericStrings%3C'a,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nu_ansi_term/display.rs.html#122\">source</a><a href=\"#impl-StructuralPartialEq-for-AnsiGenericStrings%3C'a,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"nu_ansi_term/struct.AnsiGenericStrings.html\" title=\"struct nu_ansi_term::AnsiGenericStrings\">AnsiGenericStrings</a>&lt;'a, S&gt;<div class=\"where\">where\n    &lt;S as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html#associatedtype.Owned\" title=\"type alloc::borrow::ToOwned::Owned\">Owned</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section>","StructuralPartialEq","nu_ansi_term::display::AnsiStrings","nu_ansi_term::display::AnsiByteStrings"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-AnsiGenericStrings%3C'a,+S%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/nu_ansi_term/display.rs.html#122\">source</a><a href=\"#impl-Debug-for-AnsiGenericStrings%3C'a,+S%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"nu_ansi_term/struct.AnsiGenericStrings.html\" title=\"struct nu_ansi_term::AnsiGenericStrings\">AnsiGenericStrings</a>&lt;'a, S&gt;<div class=\"where\">where\n    &lt;S as <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a>&gt;::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html#associatedtype.Owned\" title=\"type alloc::borrow::ToOwned::Owned\">Owned</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.76.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/nu_ansi_term/display.rs.html#122\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.76.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.76.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","nu_ansi_term::display::AnsiStrings","nu_ansi_term::display::AnsiByteStrings"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()