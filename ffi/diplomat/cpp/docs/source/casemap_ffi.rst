``casemap::ffi``
================

.. cpp:class:: ICU4XCaseMapper

    See the `Rust documentation for CaseMapper <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCaseMapper, ICU4XError> create(const ICU4XDataProvider& provider)

        Construct a new ICU4XCaseMapper instance for NFC

        See the `Rust documentation for new <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.new>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> lowercase_to_writeable(const std::string_view s, const ICU4XLocale& locale, W& write) const

        Returns the full lowercase mapping of the given string

        See the `Rust documentation for lowercase <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.lowercase>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> lowercase(const std::string_view s, const ICU4XLocale& locale) const

        Returns the full lowercase mapping of the given string

        See the `Rust documentation for lowercase <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.lowercase>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> uppercase_to_writeable(const std::string_view s, const ICU4XLocale& locale, W& write) const

        Returns the full uppercase mapping of the given string

        See the `Rust documentation for uppercase <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.uppercase>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> uppercase(const std::string_view s, const ICU4XLocale& locale) const

        Returns the full uppercase mapping of the given string

        See the `Rust documentation for uppercase <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.uppercase>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> titlecase_segment_to_writeable(const std::string_view s, const ICU4XLocale& locale, W& write) const

        Returns the full titlecase mapping of the given string

        See the `Rust documentation for titlecase_segment <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.titlecase_segment>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> titlecase_segment(const std::string_view s, const ICU4XLocale& locale) const

        Returns the full titlecase mapping of the given string

        See the `Rust documentation for titlecase_segment <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.titlecase_segment>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> fold_to_writeable(const std::string_view s, W& write) const

        Case-folds the characters in the given string

        See the `Rust documentation for fold <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> fold(const std::string_view s) const

        Case-folds the characters in the given string

        See the `Rust documentation for fold <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> fold_turkic_to_writeable(const std::string_view s, W& write) const

        Case-folds the characters in the given string using Turkic (T) mappings for dotted/dotless I.

        See the `Rust documentation for fold_turkic <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold_turkic>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> fold_turkic(const std::string_view s) const

        Case-folds the characters in the given string using Turkic (T) mappings for dotted/dotless I.

        See the `Rust documentation for fold_turkic <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold_turkic>`__ for more information.


    .. cpp:function:: void add_case_closure(char32_t c, ICU4XCodePointSetBuilder& builder) const

        Adds all simple case mappings and the full case folding for ``c`` to ``set``. Also adds special case closure mappings.

        In other words, this adds all characters that this casemaps to, as well as all characters that may casemap to this one.

        Note that since ICU4XCodePointSetBuilder does not handle string mappings

        See the `Rust documentation for add_case_closure <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.add_case_closure>`__ for more information.


    .. cpp:function:: bool add_string_case_closure(const std::string_view s, ICU4XCodePointSetBuilder& builder) const

        Maps the string to single code points and adds the associated case closure mappings, if they exist.

        The string is mapped to code points if it is their full case folding string. In other words, this performs a reverse full case folding and then adds the case closure items of the resulting code points. If the string is found and its closure applied, then the string itself is added as well as part of its code points' closure.

        Returns true if the string was found

        See the `Rust documentation for add_string_case_closure <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.add_string_case_closure>`__ for more information.


    .. cpp:function:: char32_t simple_lowercase(char32_t ch) const

        Returns the simple lowercase mapping of the given character.

        This function only implements simple and common mappings. Full mappings, which can map one char to a string, are not included. For full mappings, use ``ICU4XCaseMapper::lowercase``.

        See the `Rust documentation for simple_lowercase <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_lowercase>`__ for more information.


    .. cpp:function:: char32_t simple_uppercase(char32_t ch) const

        Returns the simple uppercase mapping of the given character.

        This function only implements simple and common mappings. Full mappings, which can map one char to a string, are not included. For full mappings, use ``ICU4XCaseMapper::uppercase``.

        See the `Rust documentation for simple_uppercase <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_uppercase>`__ for more information.


    .. cpp:function:: char32_t simple_titlecase(char32_t ch) const

        Returns the simple titlecase mapping of the given character.

        This function only implements simple and common mappings. Full mappings, which can map one char to a string, are not included. For full mappings, use ``ICU4XCaseMapper::titlecase_segment``.

        See the `Rust documentation for simple_titlecase <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_titlecase>`__ for more information.


    .. cpp:function:: char32_t simple_fold(char32_t ch) const

        Returns the simple casefolding of the given character.

        This function only implements simple folding. For full folding, use ``ICU4XCaseMapper::fold``.

        See the `Rust documentation for simple_fold <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_fold>`__ for more information.


    .. cpp:function:: char32_t simple_fold_turkic(char32_t ch) const

        Returns the simple casefolding of the given character in the Turkic locale

        This function only implements simple folding. For full folding, use ``ICU4XCaseMapper::fold_turkic``.

        See the `Rust documentation for simple_fold_turkic <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_fold_turkic>`__ for more information.

