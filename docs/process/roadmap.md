# ICU4X 1.0 Roadmap
The following timeline is derived from [ICU4X 1.0 PRD](./prd.md) and is optimized to provide maximal alignment and synergy between progressive feature extension, technical advancements and business evaluations. This is a preamble view. The ICU4X team is focused to deliver quality work in a way that benefits the customers and the team itself in a measurable way. We are using agile project management methodologies and our detailed work is organized in ([milestones and sprints](https://github.com/unicode-org/icu4x/milestones)). 

* [x] 2020
  * [x] October
	  * [x] 0.1 Release (#204)
  * [x] November/December
	  * [x] Post 0.1 clean up and maintenance cycle ([Milestone](https://github.com/unicode-org/icu4x/milestone/7))
* [ ] 2021
	* [x] January/February
		* [x] Locale
			* [x] Likely Subtags (#417)
		* [x] DateTime
			* [x] Day Periods (#435)
			* [x] Time zones (#418)
			* [x] Components Bag (#481)
		* [x] UnicodeSet
			* [x] L3a (#478)
	* [x] March/April
		* [x] FFI/WASM exploration (#398)
		* [x] 0.2 Release (#239)
	* [x] May
		* [x] FFI/WASM exploration (#207)
		* [x] Data Provider performance (#667)
		* [x] Gecko integration experiment
	 	* [x] Test262 evaluation	
	* [x] June-September
		* [x] 0.3 Release ([Milestone](https://github.com/unicode-org/icu4x/milestone/12))
		* [x] Unicode Regular Expressions ([#168](https://github.com/unicode-org/icu4x/issues/168))
		* [x] C++ FFI ([#674](https://github.com/unicode-org/icu4x/issues/674))
		* [x] Data provider productionization ([#873](https://github.com/unicode-org/icu4x/issues/873))
		* [x] Enable core components on `#![no_std]` ([#812](https://github.com/unicode-org/icu4x/issues/812))
		* [x] Static Data Provider (#78)
	* [x] October-December
		* [x] 0.4 Release ([Milestone](https://github.com/unicode-org/icu4x/milestone/11))
			* [x] DateTime performance (#603)
			* [x] Finish full-stack enumerated Unicode properties ([#936](https://github.com/unicode-org/icu4x/issues/936))([#1160](https://github.com/unicode-org/icu4x/issues/1160))([#1159](https://github.com/unicode-org/icu4x/issues/1159))([#1158](https://github.com/unicode-org/icu4x/issues/1158))([#1074](https://github.com/unicode-org/icu4x/issues/1074))([#1073](https://github.com/unicode-org/icu4x/issues/1073))
			* [x] ZeroVec improvements ([#1082](https://github.com/unicode-org/icu4x/issues/1082))
		* [x] 0.5 (1.0 beta) Release ([Milestone](https://github.com/unicode-org/icu4x/milestone/14))
			* [x] Full support for enumerated properties ([#148](https://github.com/unicode-org/icu4x/issues/148))
			* [x] High-fidelity code size reduction ([#874](https://github.com/unicode-org/icu4x/issues/874))
			* [x] Calendrical calculations and formatting for Japanese
			* [x] Improved data model
* [ ] 2022
	* [x] January-March
		* [x] 0.6 Release ([Milestone](https://github.com/unicode-org/icu4x/milestone/15))
			* [x] Finish full-stack Script_Extensions property ([#1577](https://github.com/unicode-org/icu4x/issues/1577))
			* [x] User-Friendly Data Slicing (part of [#1110](https://github.com/unicode-org/icu4x/issues/1110))
			* [x] Add Coptic, Indian, and Ethiopian calendars
			* [x] Fully zero-copy data
			* [x] Bi-di support in Rust
			* [x] TimeZoneFormatter
	* [ ] April-June
		* [ ] 1.0 Release ([Milestone](https://github.com/unicode-org/icu4x/milestone/16))
			* [ ] Data + API stability
			* [ ] Full FFI coverage
			* [ ] Segmenter ([#109](https://github.com/unicode-org/icu4x/issues/109)) 
			* [ ] Time Zone Calculations ([#533](https://github.com/unicode-org/icu4x/issues/533))
			* [ ] User-friendly data tooling (multi-blob, source data downloader) (part of [#1110](https://github.com/unicode-org/icu4x/issues/1110))
			* [ ] Implement locale fallbacking in data loading([#1109](https://github.com/unicode-org/icu4x/issues/1109))
			* [ ] Collator ([#971](https://github.com/unicode-org/icu4x/issues/971))
			* [ ] Calendrical calculations and formatting for Coptic, Japanese, and Indian National ([#534](https://github.com/unicode-org/icu4x/issues/534))
			* [ ] DateTimeFormat modularity / API
	* [ ] July-December
		* [ ] 1.1 Release ([Milestone](https://github.com/unicode-org/icu4x/milestone/32))
			* [ ] Implement full-stack properties of strings
			* [ ] Normalizer ([#972](https://github.com/unicode-org/icu4x/issues/972))
			* [ ] Novel DateTime Pattern Selection Algorithm ([#645](https://github.com/unicode-org/icu4x/issues/645))
			* [ ] UnicodeSet L3b ([#168](https://github.com/unicode-org/icu4x/issues/533))
		
