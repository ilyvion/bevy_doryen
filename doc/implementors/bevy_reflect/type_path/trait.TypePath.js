(function() {var implementors = {
"bevy_asset":[["impl&lt;A&gt; <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"enum\" href=\"bevy_asset/enum.Handle.html\" title=\"enum bevy_asset::Handle\">Handle</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.74.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;<a class=\"struct\" href=\"bevy_asset/struct.StrongHandle.html\" title=\"struct bevy_asset::StrongHandle\">StrongHandle</a>&gt;: <a class=\"trait\" href=\"bevy_reflect/from_reflect/trait.FromReflect.html\" title=\"trait bevy_reflect::from_reflect::FromReflect\">FromReflect</a> + <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a>,\n    <a class=\"enum\" href=\"bevy_asset/enum.AssetId.html\" title=\"enum bevy_asset::AssetId\">AssetId</a>&lt;A&gt;: <a class=\"trait\" href=\"bevy_reflect/from_reflect/trait.FromReflect.html\" title=\"trait bevy_reflect::from_reflect::FromReflect\">FromReflect</a> + <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a>,\n    A: <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> + <a class=\"trait\" href=\"bevy_asset/trait.Asset.html\" title=\"trait bevy_asset::Asset\">Asset</a>,</span>"],["impl <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_asset/struct.LoadedUntypedAsset.html\" title=\"struct bevy_asset::LoadedUntypedAsset\">LoadedUntypedAsset</a>"],["impl&lt;A&gt; <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"enum\" href=\"bevy_asset/enum.AssetId.html\" title=\"enum bevy_asset::AssetId\">AssetId</a>&lt;A&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"bevy_asset/struct.AssetIndex.html\" title=\"struct bevy_asset::AssetIndex\">AssetIndex</a>: <a class=\"trait\" href=\"bevy_reflect/from_reflect/trait.FromReflect.html\" title=\"trait bevy_reflect::from_reflect::FromReflect\">FromReflect</a> + <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a>,\n    <a class=\"struct\" href=\"uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>: <a class=\"trait\" href=\"bevy_reflect/from_reflect/trait.FromReflect.html\" title=\"trait bevy_reflect::from_reflect::FromReflect\">FromReflect</a> + <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a>,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/1.74.1/core/marker/struct.PhantomData.html\" title=\"struct core::marker::PhantomData\">PhantomData</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.1/std/primitive.fn.html\">fn</a>() -&gt; A&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/core/any/trait.Any.html\" title=\"trait core::any::Any\">Any</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    A: <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> + <a class=\"trait\" href=\"bevy_asset/trait.Asset.html\" title=\"trait bevy_asset::Asset\">Asset</a>,</span>"],["impl <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_asset/struct.AssetIndex.html\" title=\"struct bevy_asset::AssetIndex\">AssetIndex</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.1/std/primitive.u32.html\">u32</a>: <a class=\"trait\" href=\"bevy_reflect/from_reflect/trait.FromReflect.html\" title=\"trait bevy_reflect::from_reflect::FromReflect\">FromReflect</a> + <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a>,</span>"],["impl <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_asset/struct.AssetPath.html\" title=\"struct bevy_asset::AssetPath\">AssetPath</a>&lt;'static&gt;"],["impl <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_asset/struct.StrongHandle.html\" title=\"struct bevy_asset::StrongHandle\">StrongHandle</a>"],["impl <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_asset/struct.LoadedFolder.html\" title=\"struct bevy_asset::LoadedFolder\">LoadedFolder</a>"]],
"bevy_ecs":[["impl&lt;S&gt; <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_ecs/schedule/struct.State.html\" title=\"struct bevy_ecs::schedule::State\">State</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"bevy_reflect/from_reflect/trait.FromReflect.html\" title=\"trait bevy_reflect::from_reflect::FromReflect\">FromReflect</a> + <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> + <a class=\"trait\" href=\"bevy_ecs/schedule/trait.States.html\" title=\"trait bevy_ecs::schedule::States\">States</a>,</span>"],["impl&lt;S&gt; <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_ecs/schedule/struct.NextState.html\" title=\"struct bevy_ecs::schedule::NextState\">NextState</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/1.74.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;S&gt;: <a class=\"trait\" href=\"bevy_reflect/from_reflect/trait.FromReflect.html\" title=\"trait bevy_reflect::from_reflect::FromReflect\">FromReflect</a> + <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a>,\n    S: <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> + <a class=\"trait\" href=\"bevy_ecs/schedule/trait.States.html\" title=\"trait bevy_ecs::schedule::States\">States</a>,</span>"],["impl <a class=\"trait\" href=\"bevy_reflect/type_path/trait.TypePath.html\" title=\"trait bevy_reflect::type_path::TypePath\">TypePath</a> for <a class=\"struct\" href=\"bevy_ecs/entity/struct.Entity.html\" title=\"struct bevy_ecs::entity::Entity\">Entity</a>"]],
"bevy_reflect":[]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()