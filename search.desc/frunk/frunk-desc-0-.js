searchState.loadedDescShard("frunk", 0, "Frunk: generic functional programming toolbelt for Rust\nReturns a type signature for a Coproduct of the provided …\nDerives a Generic instance based on HList for a given …\nReturns a type signature for an HList of the provided types\nDerives a Generic instance based on Field + HList for a …\nModule that holds Coproduct data structures, traits, and …\nUsed for creating a Field\nThis module holds the machinery behind <code>Generic</code>.\nModule that holds HList data structures, implementations, …\nReturns an <code>HList</code> based on the values passed in.\nMacro for pattern-matching on HLists.\nTypes used for indexing into HLists and coproducts.\nThis module holds the machinery behind LabelledGeneric.\nModule for holding Monoid typeclass definitions and …\nHolds models, traits, and logic for generic traversal of …\nReturns a polymorphic function for use with …\nTraits that need to be imported for the complete <code>frunk</code> …\nModule for holding the Semigroup typeclass definition and …\nTraits that provide generic functionality for multiple …\nModule for holding Validated logic\nPhantom type for signature purposes only (has no value)\nTrait for instantiating a coproduct from an element\nTrait for extracting a value from a coproduct in an …\nEnum type representing a Coproduct. Think of this as a …\nTrait for converting a coproduct into another that can …\nTrait for folding a coproduct into a single value.\nTrait for mapping over a coproduct’s variants.\nTrait for borrowing a coproduct element by type\nTrait for extracting a subset of the possible types in a …\nTrait for retrieving a coproduct element by type\nCoproduct is either H or T, in this case, it is H\nCoproduct is either H or T, in this case, it is T\nConvert a coproduct into another that can hold its …\nConvert a coproduct into another that can hold its …\nExtract the value from a coproduct with only one variant.\nUse functions to fold a coproduct into a single value.\nUse functions to transform a Coproduct into a single value.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBorrow an element from a coproduct by type.\nBorrow an element from a coproduct by type.\nInstantiate a coproduct from an element.\nInstantiate a coproduct from an element.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUse functions to map each variant of a coproduct.\nApply a function to each variant of a Coproduct.\nExtract a subset of the possible types in a coproduct (or …\nAttempt to extract a value from a subset of the types.\nExtract a subset of the possible types in a coproduct (or …\nRetrieve an element from a coproduct by type, ignoring all …\nRetrieve an element from a coproduct by type, ignoring all …\nBorrow each variant of the <code>Coproduct</code> mutably.\nBorrow each variant of the Coproduct.\nAttempt to extract a value from a coproduct (or get the …\nAttempt to extract a value from a coproduct (or get the …\nA trait that converts from a type to a generic …\nThe generic representation type.\nConverts one type <code>Src</code> into another type <code>Dst</code> assuming they …\nConvert a value to another type provided that they have …\nConvert a value’s representation type <code>Repr</code> to the value…\nGiven a generic representation <code>Repr</code> of a <code>Dst</code>, returns <code>Dst</code>.\nConvert a value to its representation type <code>Repr</code>.\nGiven a value of type <code>Src</code>, returns its generic …\nMaps a value of a given type <code>Origin</code> using a function on a …\nMaps the given value of type <code>Self</code> by first transforming it …\nMaps a value of a given type <code>Origin</code> using a function on …\nMaps the given value of type <code>Self</code> by first transforming it …\nRepresents the most basic non-empty HList. Its value is …\nTrait for performing a left fold over an HList\nTrait for performing a right fold over an HList\nTypeclass for HList-y behaviour\nTrait for mapping over an HList\nRepresents the right-most end of a heterogeneous list\nTrait for zipping HLists\nThe 0 element in the output tuple\nTrait for transforming an HList into a nested tuple.\nReturns the length of a given HList type without making …\nIndexed type conversions of <code>T -&gt; Self</code> with index <code>I</code>. This …\nAn indexed conversion that consumes <code>self</code>, and produces a <code>T</code>…\nTrait defining extraction from a given HList\nWhat is left after you pluck the target from the Self\nTrait for pulling out some subset of an HList, using type …\nTrait for borrowing an HList element by type\nThe 1 element in the output tuple\nExtend the contents of this HList with another HList\nExtend the contents of this HList with another HList\nPerform a left fold over an HList.\nPerform a left fold over an HList.\nPerform a left fold over an HList.\nPerform a right fold over an HList.\nPerform a right fold over an HList.\nPerform a right fold over an HList.\n<code>HFoldRightable</code> inner mechanics for folding with a folder …\nReturns the argument unchanged.\nReturns the argument unchanged.\nBorrow an element by type from an HList.\nBorrow an element by type from an HList.\nMutably borrow an element by type from an HList.\nMutably borrow an element by type from an HList.\nTakes an element and an Hlist and returns another one with …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReverse the HList.\nReverse the HList.\nTurns an HList into nested Tuple2s, which are less …\nTurns an HList into nested Tuple2s, which are less …\nReturns whether a given HList is empty\nReturns whether a given HList is empty\nReturns whether a given HList is empty\nReturns the length of a given HList\nReturns the length of a given HList\nReturns the length of a given HList\nFree function version of <code>LiftFrom::lift_from</code>.\nPerforms the indexed conversion.\nPerforms the indexed conversion.\nApply a function to each element of an HList.\nApply a function to each element of an HList.\nApply a function to each element of an HList.\nRemove an element by type from an HList.\nRemove an element by type from an HList.\nReturns the head of the list and the tail of the list as a …\nPrepends an item to the current HList\nPrepend an item to the current HList\nPrepend an item to the current HList\nConsumes the current HList and returns an HList with the …\nConsume the current HList and return an HList with the …\nConsume the current HList and return an HList with the …\nReturns the length of a given HList type without making …\nReturn an <code>HList</code> where the contents are mutable references …\nReturn an <code>HList</code> where the contents are mutable references …\nReturn an HList where the contents are references to the …\nReturn an HList where the contents are references to the …\nZip this HList with another one.\nZip two HLists together.\nZip two HLists together.\nA real <code>foldr</code> for the folder that must be owned to fold.\nIndex for the case where we need to do work in order to …\nUsed as an index into an <code>HList</code>.\nIndex for the case where we don’t need to do any …\nIndex type wrapper for transmogrifying a generic Source to …\nIndex type wrapper for transmogrifying through a (known) …\nIndex type wrapper for transmogrifying a generic plucked …\nAn index denoting that <code>Suffix</code> is just that.\nUsed as an index into an <code>HList</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTrait for plucking out a <code>Field</code> from a type by type-level …\nA Label contains a type-level Name, a runtime value, and a …\nTrait for turning a Field HList into an un-labelled HList\nA trait that strips type-level strings from the labels\nA trait that converts from a type to a labelled generic …\nThe labelled generic representation type.\nThe labelled generic representation type.\nTrait for transmogrifying a <code>Source</code> type into a <code>Target</code> type.\nA version of Field that doesn’t have a type-level label, …\nTypes for building type-level labels from character …\nConvert from one type to another using a type with the same\nReturns a new Field for a given value and custom name.\nConvert a value’s labelled representation type <code>Repr</code> to …\nReturns the argument unchanged.\nReturns the argument unchanged.\nGiven a labelled generic representation of a <code>Dst</code>, returns …\nConvert a value to its representation type <code>Repr</code>.\nConvert a value to its representation type <code>Repr</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGiven a <code>Src</code>, returns its labelled generic representation.\nTurns the current HList into an unlabelled one.\nTurns the current HList into a value-labelled one.\nConverts one type into another assuming they have the same …\nReturns a pair consisting of the value pointed to by the …\nConverts from one type into another assuming that their …\nConverts from another type A into Self assuming that A and …\nConverts from one type into another assuming that their …\nConverts from another type <code>Src</code> into <code>Self</code> assuming that <code>Src</code> …\nConsume this current object and return an object of the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nA Monoid is a Semigroup that has an empty/ zero value\nGiven a sequence of <code>xs</code>, combine them and return the total\nReturn this combined with itself <code>n</code> times.\nFor a given Monoid, returns its empty/zero value\nTrait for traversing based on Path\nReturns the argument unchanged.\nReturns a pair consisting of the value pointed to by the …\nGets something using the current path\nCalls <code>U::from(self)</code>.\nCreates a new Path\nWrapper type for boolean that acts as a bitwise &amp;&amp; …\nWrapper type for boolean that acts as a bitwise || …\nWrapper type for types that are ordered and can have a Max …\nWrapper type for types that are ordered and can have a Min …\nWrapper type for types that can have a Product combination\nA Semigroup is a class of thing that has a definable …\nAssociative operation taking which combines two values.\nGiven a sequence of <code>xs</code>, combine them and return the total\nReturn this combined with itself <code>n</code> times.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThis is a simple, user-implementable alternative to <code>Fn</code>.\nTrait that allows for reversing a given data structure.\nWrapper type around a function for polymorphic maps and …\nAn alternative to <code>AsMut</code> that does not force the reference …\nAn alternative to AsRef that does not force the reference …\nCall the <code>Func</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReverses a given data structure.\nTrait for “lifting” a given type into a Validated\nA Validated is either an Ok holding an HList or an Err, …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nTurns this Validated into a Result.\nConsumes the current Result into a Validated so that we …\nReturns true if this validation is Err, false otherwise\nReturns true if this validation is Ok, false otherwise")