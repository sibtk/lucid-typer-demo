//! Thesaurus module for realistic word substitution during typing simulation.
//!
//! Provides O(1) lookup of synonyms for common English words, with
//! case-insensitive matching and capitalization preservation.

use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashMap;

/// Static thesaurus data: lowercase word -> slice of synonyms.
static THESAURUS: Lazy<HashMap<&'static str, &'static [&'static str]>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // =========================================================================
    // ADJECTIVES - Descriptive words
    // =========================================================================

    // Quality/Evaluation
    m.insert("good", &["great", "excellent", "fine", "nice", "solid", "quality"][..]);
    m.insert("bad", &["poor", "terrible", "awful", "negative", "problematic"][..]);
    m.insert("great", &["excellent", "fantastic", "wonderful", "amazing", "outstanding"][..]);
    m.insert("nice", &["pleasant", "lovely", "good", "fine", "agreeable"][..]);
    m.insert("beautiful", &["lovely", "gorgeous", "stunning", "attractive", "pretty"][..]);
    m.insert("ugly", &["unattractive", "unsightly", "hideous", "unpleasant"][..]);
    m.insert("perfect", &["ideal", "flawless", "excellent", "optimal"][..]);

    // Size
    m.insert("big", &["large", "huge", "significant", "major", "substantial", "considerable"][..]);
    m.insert("small", &["little", "tiny", "minor", "slight", "modest", "limited"][..]);
    m.insert("large", &["big", "huge", "massive", "extensive", "substantial"][..]);
    m.insert("huge", &["massive", "enormous", "immense", "vast", "gigantic"][..]);
    m.insert("tiny", &["small", "little", "minute", "miniature"][..]);

    // Importance
    m.insert("important", &["significant", "crucial", "essential", "vital", "critical", "key"][..]);
    m.insert("significant", &["important", "notable", "considerable", "substantial", "meaningful"][..]);
    m.insert("major", &["significant", "important", "substantial", "considerable", "primary"][..]);
    m.insert("minor", &["small", "slight", "trivial", "insignificant", "secondary"][..]);
    m.insert("crucial", &["critical", "vital", "essential", "key", "pivotal"][..]);

    // Difficulty
    m.insert("easy", &["simple", "straightforward", "effortless", "uncomplicated"][..]);
    m.insert("hard", &["difficult", "challenging", "tough", "complex", "demanding"][..]);
    m.insert("difficult", &["hard", "challenging", "tough", "complex", "demanding"][..]);
    m.insert("simple", &["easy", "straightforward", "basic", "uncomplicated"][..]);
    m.insert("complex", &["complicated", "intricate", "sophisticated", "elaborate"][..]);

    // Speed/Time
    m.insert("fast", &["quick", "rapid", "swift", "speedy", "prompt"][..]);
    m.insert("slow", &["gradual", "unhurried", "leisurely", "sluggish"][..]);
    m.insert("quick", &["fast", "rapid", "swift", "prompt", "speedy"][..]);
    m.insert("new", &["recent", "modern", "fresh", "novel", "current"][..]);
    m.insert("old", &["ancient", "aged", "outdated", "previous", "former"][..]);

    // Certainty
    m.insert("clear", &["obvious", "evident", "apparent", "plain", "distinct"][..]);
    m.insert("obvious", &["clear", "evident", "apparent", "plain", "unmistakable"][..]);
    m.insert("certain", &["sure", "definite", "confident", "convinced", "positive"][..]);
    m.insert("possible", &["potential", "feasible", "likely", "conceivable"][..]);
    m.insert("likely", &["probable", "possible", "expected", "anticipated"][..]);

    // Other adjectives
    m.insert("different", &["various", "diverse", "distinct", "varied", "alternative"][..]);
    m.insert("same", &["identical", "similar", "equivalent", "equal", "matching"][..]);
    m.insert("similar", &["alike", "comparable", "related", "analogous"][..]);
    m.insert("specific", &["particular", "precise", "exact", "definite", "certain"][..]);
    m.insert("general", &["overall", "broad", "common", "widespread", "universal"][..]);
    m.insert("main", &["primary", "principal", "chief", "major", "central"][..]);
    m.insert("real", &["actual", "genuine", "true", "authentic", "legitimate"][..]);
    m.insert("true", &["accurate", "correct", "real", "genuine", "valid"][..]);
    m.insert("false", &["incorrect", "wrong", "untrue", "inaccurate", "erroneous"][..]);
    m.insert("whole", &["entire", "complete", "full", "total"][..]);
    m.insert("strong", &["powerful", "robust", "solid", "intense", "forceful"][..]);
    m.insert("weak", &["feeble", "fragile", "frail", "poor", "insufficient"][..]);

    // =========================================================================
    // VERBS - Action words
    // =========================================================================

    // Communication
    m.insert("said", &["stated", "mentioned", "explained", "noted", "remarked", "expressed"][..]);
    m.insert("say", &["state", "mention", "express", "declare", "indicate", "note"][..]);
    m.insert("tell", &["inform", "notify", "advise", "explain", "describe"][..]);
    m.insert("ask", &["inquire", "question", "request", "query"][..]);
    m.insert("answer", &["respond", "reply", "address", "explain"][..]);
    m.insert("explain", &["describe", "clarify", "elaborate", "illustrate", "demonstrate"][..]);
    m.insert("describe", &["explain", "depict", "portray", "illustrate", "outline"][..]);
    m.insert("suggest", &["propose", "recommend", "imply", "indicate", "hint"][..]);
    m.insert("argue", &["contend", "assert", "claim", "maintain", "debate"][..]);
    m.insert("claim", &["assert", "argue", "maintain", "state", "declare"][..]);
    m.insert("discuss", &["examine", "explore", "analyze", "consider", "debate"][..]);

    // Thinking/Belief
    m.insert("think", &["believe", "consider", "feel", "suppose", "reckon", "assume"][..]);
    m.insert("believe", &["think", "consider", "feel", "suppose", "assume", "trust"][..]);
    m.insert("know", &["understand", "realize", "recognize", "comprehend", "grasp"][..]);
    m.insert("understand", &["comprehend", "grasp", "realize", "recognize", "appreciate"][..]);
    m.insert("consider", &["think about", "contemplate", "examine", "evaluate", "weigh"][..]);
    m.insert("realize", &["understand", "recognize", "discover", "notice", "grasp"][..]);
    m.insert("assume", &["suppose", "presume", "believe", "expect", "think"][..]);
    m.insert("remember", &["recall", "recollect", "retain", "recognize"][..]);
    m.insert("forget", &["overlook", "neglect", "disregard", "ignore"][..]);

    // Action/Creation
    m.insert("make", &["create", "produce", "develop", "build", "construct", "generate"][..]);
    m.insert("create", &["make", "produce", "develop", "generate", "establish", "form"][..]);
    m.insert("build", &["construct", "create", "develop", "establish", "form"][..]);
    m.insert("develop", &["create", "build", "establish", "advance", "evolve", "grow"][..]);
    m.insert("use", &["utilize", "employ", "apply", "implement", "leverage"][..]);
    m.insert("work", &["function", "operate", "perform", "labor", "serve"][..]);
    m.insert("help", &["assist", "aid", "support", "facilitate", "enable"][..]);
    m.insert("try", &["attempt", "endeavor", "strive", "seek", "aim"][..]);
    m.insert("start", &["begin", "commence", "initiate", "launch", "embark"][..]);
    m.insert("begin", &["start", "commence", "initiate", "launch", "originate"][..]);
    m.insert("end", &["finish", "conclude", "complete", "terminate", "cease"][..]);
    m.insert("finish", &["complete", "end", "conclude", "finalize", "accomplish"][..]);
    m.insert("stop", &["cease", "halt", "discontinue", "terminate", "end"][..]);
    m.insert("continue", &["proceed", "persist", "carry on", "maintain", "sustain"][..]);
    m.insert("change", &["alter", "modify", "transform", "adjust", "shift"][..]);
    m.insert("improve", &["enhance", "better", "upgrade", "advance", "refine"][..]);
    m.insert("increase", &["grow", "expand", "rise", "raise", "boost", "elevate"][..]);
    m.insert("decrease", &["reduce", "lower", "diminish", "decline", "drop"][..]);
    m.insert("reduce", &["decrease", "lower", "diminish", "cut", "minimize"][..]);
    m.insert("add", &["include", "incorporate", "insert", "attach", "append"][..]);
    m.insert("remove", &["eliminate", "delete", "take away", "extract", "exclude"][..]);

    // Observation/Discovery
    m.insert("show", &["demonstrate", "indicate", "reveal", "display", "illustrate", "prove"][..]);
    m.insert("see", &["observe", "notice", "view", "witness", "perceive", "spot"][..]);
    m.insert("find", &["discover", "locate", "identify", "detect", "uncover"][..]);
    m.insert("look", &["examine", "observe", "view", "inspect", "study"][..]);
    m.insert("watch", &["observe", "monitor", "view", "witness", "follow"][..]);
    m.insert("notice", &["observe", "see", "detect", "recognize", "spot"][..]);

    // Possession/Acquisition
    m.insert("get", &["obtain", "receive", "acquire", "gain", "secure", "attain"][..]);
    m.insert("give", &["provide", "offer", "present", "supply", "deliver", "grant"][..]);
    m.insert("take", &["grab", "seize", "acquire", "obtain", "accept"][..]);
    m.insert("have", &["possess", "own", "hold", "maintain", "contain"][..]);
    m.insert("keep", &["retain", "maintain", "preserve", "hold", "save"][..]);
    m.insert("need", &["require", "demand", "necessitate", "want"][..]);
    m.insert("want", &["desire", "wish", "seek", "prefer", "need"][..]);

    // Movement
    m.insert("go", &["move", "proceed", "travel", "head", "advance"][..]);
    m.insert("come", &["arrive", "approach", "reach", "appear"][..]);
    m.insert("move", &["shift", "transfer", "relocate", "proceed", "advance"][..]);
    m.insert("put", &["place", "set", "position", "locate", "lay"][..]);
    m.insert("bring", &["carry", "deliver", "transport", "convey", "fetch"][..]);
    m.insert("lead", &["guide", "direct", "head", "conduct", "steer"][..]);
    m.insert("follow", &["pursue", "trail", "accompany", "succeed", "obey"][..]);
    m.insert("run", &["operate", "manage", "execute", "conduct", "administer"][..]);

    // Other verbs
    m.insert("allow", &["permit", "enable", "let", "authorize", "grant"][..]);
    m.insert("prevent", &["stop", "avoid", "block", "hinder", "prohibit"][..]);
    m.insert("cause", &["create", "produce", "generate", "trigger", "induce"][..]);
    m.insert("affect", &["influence", "impact", "alter", "change", "shape"][..]);
    m.insert("include", &["contain", "comprise", "incorporate", "encompass", "cover"][..]);
    m.insert("involve", &["include", "require", "entail", "encompass", "engage"][..]);
    m.insert("require", &["need", "demand", "necessitate", "call for"][..]);
    m.insert("provide", &["give", "supply", "offer", "deliver", "furnish"][..]);
    m.insert("support", &["assist", "help", "back", "uphold", "sustain"][..]);
    m.insert("ensure", &["guarantee", "secure", "confirm", "verify", "assure"][..]);
    m.insert("achieve", &["accomplish", "attain", "reach", "realize", "gain"][..]);
    m.insert("fail", &["fall short", "miss", "falter", "flounder"][..]);
    m.insert("succeed", &["achieve", "accomplish", "prosper", "thrive", "triumph"][..]);

    // =========================================================================
    // ADVERBS - Modifiers
    // =========================================================================

    m.insert("very", &["extremely", "highly", "really", "particularly", "especially", "quite"][..]);
    m.insert("really", &["truly", "actually", "genuinely", "indeed", "certainly"][..]);
    m.insert("also", &["additionally", "furthermore", "moreover", "too", "as well"][..]);
    m.insert("often", &["frequently", "regularly", "commonly", "repeatedly"][..]);
    m.insert("always", &["constantly", "continually", "perpetually", "invariably"][..]);
    m.insert("never", &["not ever", "at no time", "not once"][..]);
    m.insert("usually", &["typically", "generally", "normally", "commonly", "ordinarily"][..]);
    m.insert("sometimes", &["occasionally", "periodically", "at times", "now and then"][..]);
    m.insert("especially", &["particularly", "specifically", "notably", "mainly"][..]);
    m.insert("probably", &["likely", "perhaps", "possibly", "presumably"][..]);
    m.insert("actually", &["really", "in fact", "truly", "indeed", "genuinely"][..]);
    m.insert("basically", &["essentially", "fundamentally", "primarily", "mainly"][..]);
    m.insert("finally", &["ultimately", "eventually", "lastly", "in the end"][..]);
    m.insert("quickly", &["rapidly", "swiftly", "promptly", "speedily", "fast"][..]);
    m.insert("slowly", &["gradually", "steadily", "leisurely", "unhurriedly"][..]);
    m.insert("completely", &["entirely", "fully", "totally", "wholly", "absolutely"][..]);
    m.insert("slightly", &["somewhat", "a little", "marginally", "mildly", "faintly"][..]);

    // =========================================================================
    // CONJUNCTIONS & TRANSITIONS
    // =========================================================================

    m.insert("however", &["nevertheless", "nonetheless", "yet", "still", "though", "but"][..]);
    m.insert("therefore", &["thus", "consequently", "hence", "accordingly", "so"][..]);
    m.insert("because", &["since", "as", "given that", "due to the fact that"][..]);
    m.insert("although", &["though", "even though", "while", "whereas", "despite"][..]);
    m.insert("while", &["whereas", "although", "though", "whilst"][..]);
    m.insert("but", &["however", "yet", "although", "though", "nevertheless"][..]);
    m.insert("so", &["therefore", "thus", "consequently", "hence", "accordingly"][..]);
    m.insert("and", &["as well as", "along with", "plus", "in addition to"][..]);
    m.insert("or", &["alternatively", "otherwise", "either"][..]);

    // =========================================================================
    // NOUNS - Common nouns
    // =========================================================================

    m.insert("problem", &["issue", "challenge", "difficulty", "concern", "obstacle"][..]);
    m.insert("issue", &["problem", "matter", "concern", "topic", "question"][..]);
    m.insert("thing", &["item", "object", "matter", "element", "aspect"][..]);
    m.insert("way", &["method", "approach", "manner", "means", "technique"][..]);
    m.insert("idea", &["concept", "notion", "thought", "theory", "view"][..]);
    m.insert("reason", &["cause", "explanation", "rationale", "justification", "basis"][..]);
    m.insert("result", &["outcome", "consequence", "effect", "conclusion", "product"][..]);
    m.insert("effect", &["impact", "result", "consequence", "influence", "outcome"][..]);
    m.insert("impact", &["effect", "influence", "consequence", "result"][..]);
    m.insert("difference", &["distinction", "variation", "contrast", "disparity"][..]);
    m.insert("example", &["instance", "illustration", "case", "sample", "demonstration"][..]);
    m.insert("fact", &["reality", "truth", "detail", "point", "datum"][..]);
    m.insert("point", &["aspect", "matter", "issue", "argument", "factor"][..]);
    m.insert("part", &["portion", "section", "component", "element", "segment"][..]);
    m.insert("area", &["region", "field", "domain", "sector", "zone"][..]);
    m.insert("place", &["location", "spot", "site", "area", "position"][..]);
    m.insert("time", &["period", "moment", "occasion", "era", "duration"][..]);
    m.insert("year", &["period", "time", "era", "season"][..]);
    m.insert("people", &["individuals", "persons", "humans", "population", "society"][..]);
    m.insert("person", &["individual", "human", "someone", "somebody"][..]);
    m.insert("group", &["team", "collection", "set", "cluster", "assembly"][..]);
    m.insert("number", &["amount", "quantity", "figure", "total", "count"][..]);
    m.insert("level", &["degree", "extent", "amount", "stage", "tier"][..]);
    m.insert("study", &["research", "investigation", "analysis", "examination", "survey"][..]);
    m.insert("research", &["study", "investigation", "analysis", "inquiry", "exploration"][..]);
    m.insert("information", &["data", "details", "facts", "knowledge", "intelligence"][..]);
    m.insert("data", &["information", "facts", "figures", "statistics", "evidence"][..]);
    m.insert("evidence", &["proof", "data", "support", "documentation", "indication"][..]);

    // =========================================================================
    // ADDITIONAL COMMON WORDS - High frequency in essays
    // =========================================================================

    // More adjectives
    m.insert("common", &["typical", "usual", "frequent", "widespread", "prevalent"][..]);
    m.insert("rare", &["uncommon", "unusual", "scarce", "infrequent", "exceptional"][..]);
    m.insert("popular", &["widespread", "common", "well-known", "famous", "fashionable"][..]);
    m.insert("famous", &["well-known", "renowned", "celebrated", "notable", "prominent"][..]);
    m.insert("strange", &["odd", "unusual", "peculiar", "weird", "curious"][..]);
    m.insert("normal", &["typical", "usual", "standard", "ordinary", "regular"][..]);
    m.insert("special", &["unique", "distinctive", "particular", "exceptional", "remarkable"][..]);
    m.insert("natural", &["normal", "typical", "organic", "inherent", "innate"][..]);
    m.insert("artificial", &["synthetic", "man-made", "manufactured", "fake", "false"][..]);
    m.insert("modern", &["contemporary", "current", "present-day", "recent", "up-to-date"][..]);
    m.insert("ancient", &["old", "historic", "antique", "age-old", "primeval"][..]);
    m.insert("traditional", &["conventional", "classic", "customary", "established", "orthodox"][..]);
    m.insert("original", &["initial", "first", "primary", "authentic", "novel"][..]);
    m.insert("final", &["last", "ultimate", "concluding", "terminal", "eventual"][..]);
    m.insert("initial", &["first", "original", "starting", "beginning", "opening"][..]);
    m.insert("previous", &["prior", "former", "earlier", "preceding", "past"][..]);
    m.insert("next", &["following", "subsequent", "upcoming", "succeeding"][..]);
    m.insert("current", &["present", "existing", "ongoing", "contemporary", "modern"][..]);
    m.insert("recent", &["latest", "new", "current", "modern", "fresh"][..]);
    m.insert("future", &["upcoming", "forthcoming", "prospective", "eventual", "later"][..]);
    m.insert("past", &["previous", "former", "earlier", "prior", "bygone"][..]);
    m.insert("present", &["current", "existing", "contemporary", "immediate"][..]);
    m.insert("necessary", &["essential", "required", "needed", "vital", "mandatory"][..]);
    m.insert("sufficient", &["enough", "adequate", "ample", "satisfactory"][..]);
    m.insert("available", &["accessible", "obtainable", "at hand", "ready"][..]);
    m.insert("useful", &["helpful", "beneficial", "practical", "valuable", "handy"][..]);
    m.insert("effective", &["successful", "efficient", "productive", "powerful", "potent"][..]);
    m.insert("efficient", &["effective", "productive", "economical", "streamlined"][..]);
    m.insert("successful", &["effective", "prosperous", "thriving", "triumphant"][..]);
    m.insert("appropriate", &["suitable", "proper", "fitting", "relevant", "apt"][..]);
    m.insert("relevant", &["pertinent", "applicable", "related", "appropriate", "connected"][..]);
    m.insert("positive", &["favorable", "beneficial", "good", "constructive", "optimistic"][..]);
    m.insert("negative", &["adverse", "harmful", "bad", "unfavorable", "detrimental"][..]);
    m.insert("interesting", &["fascinating", "intriguing", "engaging", "compelling", "captivating"][..]);
    m.insert("boring", &["dull", "tedious", "monotonous", "uninteresting", "tiresome"][..]);
    m.insert("exciting", &["thrilling", "stimulating", "exhilarating", "electrifying"][..]);
    m.insert("surprising", &["unexpected", "astonishing", "startling", "remarkable", "shocking"][..]);
    m.insert("expected", &["anticipated", "predicted", "foreseen", "projected"][..]);
    m.insert("unexpected", &["surprising", "unforeseen", "unanticipated", "sudden"][..]);
    m.insert("public", &["communal", "shared", "collective", "common", "open"][..]);
    m.insert("private", &["personal", "individual", "confidential", "secret", "exclusive"][..]);
    m.insert("open", &["accessible", "available", "unrestricted", "public", "transparent"][..]);
    m.insert("closed", &["shut", "sealed", "restricted", "inaccessible"][..]);
    m.insert("full", &["complete", "entire", "filled", "packed", "loaded"][..]);
    m.insert("empty", &["vacant", "bare", "hollow", "void", "blank"][..]);
    m.insert("rich", &["wealthy", "affluent", "prosperous", "abundant", "plentiful"][..]);
    m.insert("poor", &["impoverished", "deprived", "needy", "lacking", "inadequate"][..]);
    m.insert("high", &["elevated", "tall", "lofty", "raised", "increased"][..]);
    m.insert("low", &["reduced", "decreased", "minimal", "small", "inferior"][..]);
    m.insert("long", &["extended", "lengthy", "prolonged", "extensive", "enduring"][..]);
    m.insert("short", &["brief", "concise", "limited", "abbreviated", "quick"][..]);
    m.insert("wide", &["broad", "extensive", "expansive", "vast", "spacious"][..]);
    m.insert("narrow", &["limited", "restricted", "confined", "tight", "slim"][..]);
    m.insert("deep", &["profound", "intense", "thorough", "extensive", "serious"][..]);
    m.insert("shallow", &["superficial", "surface-level", "slight", "limited"][..]);
    m.insert("broad", &["wide", "extensive", "comprehensive", "general", "expansive"][..]);
    m.insert("basic", &["fundamental", "essential", "elementary", "primary", "core"][..]);
    m.insert("advanced", &["sophisticated", "complex", "developed", "progressive"][..]);
    m.insert("serious", &["grave", "severe", "critical", "significant", "important"][..]);
    m.insert("careful", &["cautious", "thorough", "attentive", "meticulous", "precise"][..]);
    m.insert("particular", &["specific", "certain", "distinct", "individual", "precise"][..]);
    m.insert("entire", &["whole", "complete", "full", "total", "comprehensive"][..]);
    m.insert("complete", &["entire", "whole", "full", "total", "finished"][..]);
    m.insert("partial", &["incomplete", "limited", "fragmentary", "fractional"][..]);
    m.insert("extreme", &["severe", "intense", "drastic", "radical", "excessive"][..]);
    m.insert("moderate", &["reasonable", "modest", "limited", "mild", "temperate"][..]);
    m.insert("average", &["typical", "ordinary", "normal", "standard", "medium"][..]);
    m.insert("proper", &["appropriate", "correct", "suitable", "fitting", "right"][..]);
    m.insert("wrong", &["incorrect", "mistaken", "erroneous", "inaccurate", "improper"][..]);
    m.insert("right", &["correct", "accurate", "proper", "appropriate", "just"][..]);
    m.insert("correct", &["right", "accurate", "proper", "exact", "precise"][..]);
    m.insert("accurate", &["correct", "precise", "exact", "true", "faithful"][..]);
    m.insert("precise", &["exact", "accurate", "specific", "definite", "detailed"][..]);
    m.insert("exact", &["precise", "accurate", "specific", "correct", "identical"][..]);
    m.insert("rough", &["approximate", "estimated", "imprecise", "coarse"][..]);
    m.insert("equal", &["equivalent", "identical", "same", "matching", "comparable"][..]);
    m.insert("fair", &["just", "equitable", "reasonable", "impartial", "balanced"][..]);
    m.insert("safe", &["secure", "protected", "harmless", "risk-free"][..]);
    m.insert("dangerous", &["hazardous", "risky", "perilous", "unsafe", "threatening"][..]);
    m.insert("healthy", &["fit", "well", "sound", "robust", "wholesome"][..]);
    m.insert("physical", &["bodily", "material", "tangible", "corporeal"][..]);
    m.insert("mental", &["psychological", "cognitive", "intellectual", "emotional"][..]);
    m.insert("social", &["communal", "collective", "public", "interpersonal", "societal"][..]);
    m.insert("economic", &["financial", "monetary", "fiscal", "commercial"][..]);
    m.insert("political", &["governmental", "civic", "public", "diplomatic"][..]);
    m.insert("cultural", &["social", "ethnic", "artistic", "civilizational"][..]);
    m.insert("environmental", &["ecological", "natural", "green", "climatic"][..]);
    m.insert("technical", &["technological", "mechanical", "scientific", "specialized"][..]);
    m.insert("scientific", &["empirical", "systematic", "analytical", "methodical"][..]);
    m.insert("legal", &["lawful", "legitimate", "judicial", "statutory"][..]);
    m.insert("official", &["formal", "authorized", "sanctioned", "legitimate"][..]);
    m.insert("personal", &["individual", "private", "own", "subjective"][..]);
    m.insert("professional", &["expert", "skilled", "qualified", "occupational"][..]);

    // More verbs
    m.insert("like", &["enjoy", "appreciate", "prefer", "favor", "love"][..]);
    m.insert("love", &["adore", "cherish", "treasure", "appreciate", "enjoy"][..]);
    m.insert("hate", &["despise", "detest", "loathe", "dislike", "abhor"][..]);
    m.insert("enjoy", &["like", "appreciate", "relish", "savor", "love"][..]);
    m.insert("prefer", &["favor", "choose", "like", "opt for"][..]);
    m.insert("choose", &["select", "pick", "opt for", "decide on", "prefer"][..]);
    m.insert("decide", &["determine", "choose", "resolve", "conclude", "settle"][..]);
    m.insert("plan", &["intend", "propose", "design", "arrange", "organize"][..]);
    m.insert("prepare", &["ready", "arrange", "organize", "set up", "plan"][..]);
    m.insert("expect", &["anticipate", "predict", "foresee", "await", "hope"][..]);
    m.insert("hope", &["wish", "expect", "desire", "aspire", "anticipate"][..]);
    m.insert("wish", &["want", "desire", "hope", "long for"][..]);
    m.insert("wait", &["await", "expect", "anticipate", "stay"][..]);
    m.insert("leave", &["depart", "exit", "go", "abandon", "vacate"][..]);
    m.insert("stay", &["remain", "continue", "linger", "wait", "persist"][..]);
    m.insert("return", &["come back", "go back", "restore", "give back"][..]);
    m.insert("enter", &["go into", "access", "join", "begin", "start"][..]);
    m.insert("join", &["connect", "unite", "combine", "link", "merge"][..]);
    m.insert("separate", &["divide", "split", "disconnect", "detach", "isolate"][..]);
    m.insert("connect", &["link", "join", "unite", "attach", "relate"][..]);
    m.insert("combine", &["merge", "unite", "join", "blend", "integrate"][..]);
    m.insert("divide", &["separate", "split", "partition", "distribute"][..]);
    m.insert("share", &["distribute", "divide", "split", "give", "provide"][..]);
    m.insert("spread", &["distribute", "disperse", "extend", "expand", "scatter"][..]);
    m.insert("grow", &["increase", "expand", "develop", "rise", "flourish"][..]);
    m.insert("shrink", &["decrease", "reduce", "contract", "diminish", "decline"][..]);
    m.insert("expand", &["grow", "extend", "increase", "enlarge", "broaden"][..]);
    m.insert("extend", &["expand", "stretch", "prolong", "lengthen", "continue"][..]);
    m.insert("rise", &["increase", "grow", "ascend", "climb", "elevate"][..]);
    m.insert("fall", &["drop", "decline", "decrease", "descend", "plummet"][..]);
    m.insert("raise", &["lift", "increase", "elevate", "boost", "heighten"][..]);
    m.insert("lower", &["reduce", "decrease", "drop", "diminish", "cut"][..]);
    m.insert("produce", &["create", "make", "generate", "manufacture", "yield"][..]);
    m.insert("consume", &["use", "spend", "eat", "utilize", "deplete"][..]);
    m.insert("buy", &["purchase", "acquire", "obtain", "get", "procure"][..]);
    m.insert("sell", &["market", "trade", "vend", "offer", "dispose of"][..]);
    m.insert("spend", &["use", "expend", "consume", "invest", "pay"][..]);
    m.insert("save", &["preserve", "conserve", "keep", "protect", "store"][..]);
    m.insert("earn", &["make", "gain", "acquire", "obtain", "receive"][..]);
    m.insert("pay", &["compensate", "reimburse", "settle", "remunerate"][..]);
    m.insert("cost", &["price", "charge", "amount to", "total"][..]);
    m.insert("reach", &["arrive at", "attain", "achieve", "get to", "access"][..]);
    m.insert("arrive", &["come", "reach", "get to", "appear", "show up"][..]);
    m.insert("depart", &["leave", "go", "exit", "set off", "withdraw"][..]);
    m.insert("exist", &["live", "be", "occur", "survive", "persist"][..]);
    m.insert("appear", &["seem", "emerge", "show up", "surface", "materialize"][..]);
    m.insert("disappear", &["vanish", "fade", "go away", "cease"][..]);
    m.insert("occur", &["happen", "take place", "arise", "emerge", "transpire"][..]);
    m.insert("happen", &["occur", "take place", "arise", "transpire", "unfold"][..]);
    m.insert("remain", &["stay", "continue", "persist", "endure", "last"][..]);
    m.insert("last", &["continue", "endure", "persist", "remain", "survive"][..]);
    m.insert("survive", &["endure", "last", "persist", "outlive", "withstand"][..]);
    m.insert("face", &["confront", "encounter", "meet", "experience", "deal with"][..]);
    m.insert("meet", &["encounter", "face", "confront", "join", "satisfy"][..]);
    m.insert("avoid", &["evade", "escape", "prevent", "dodge", "shun"][..]);
    m.insert("seek", &["search for", "look for", "pursue", "try", "attempt"][..]);
    m.insert("search", &["look for", "seek", "hunt", "explore", "investigate"][..]);
    m.insert("discover", &["find", "uncover", "detect", "reveal", "learn"][..]);
    m.insert("recognize", &["identify", "acknowledge", "realize", "notice", "know"][..]);
    m.insert("identify", &["recognize", "determine", "detect", "pinpoint", "distinguish"][..]);
    m.insert("determine", &["decide", "establish", "ascertain", "figure out", "conclude"][..]);
    m.insert("establish", &["create", "found", "set up", "form", "institute"][..]);
    m.insert("maintain", &["keep", "preserve", "sustain", "continue", "uphold"][..]);
    m.insert("protect", &["defend", "guard", "shield", "preserve", "safeguard"][..]);
    m.insert("defend", &["protect", "guard", "shield", "support", "justify"][..]);
    m.insert("attack", &["assault", "strike", "criticize", "challenge", "target"][..]);
    m.insert("damage", &["harm", "hurt", "injure", "impair", "ruin"][..]);
    m.insert("destroy", &["ruin", "demolish", "wreck", "eliminate", "devastate"][..]);
    m.insert("repair", &["fix", "restore", "mend", "correct", "remedy"][..]);
    m.insert("replace", &["substitute", "exchange", "swap", "change", "supersede"][..]);
    m.insert("restore", &["repair", "return", "renew", "revive", "reinstate"][..]);
    m.insert("form", &["create", "shape", "develop", "establish", "constitute"][..]);
    m.insert("design", &["create", "plan", "develop", "devise", "conceive"][..]);
    m.insert("test", &["examine", "check", "evaluate", "assess", "try"][..]);
    m.insert("examine", &["inspect", "study", "analyze", "investigate", "review"][..]);
    m.insert("check", &["verify", "examine", "inspect", "confirm", "test"][..]);
    m.insert("measure", &["assess", "evaluate", "gauge", "calculate", "determine"][..]);
    m.insert("compare", &["contrast", "relate", "match", "equate", "liken"][..]);
    m.insert("contrast", &["compare", "differ", "distinguish", "oppose"][..]);
    m.insert("match", &["correspond", "fit", "equal", "compare", "suit"][..]);
    m.insert("differ", &["vary", "contrast", "diverge", "disagree", "deviate"][..]);
    m.insert("vary", &["differ", "change", "fluctuate", "alternate", "range"][..]);
    m.insert("accept", &["receive", "take", "agree to", "acknowledge", "approve"][..]);
    m.insert("reject", &["refuse", "decline", "deny", "dismiss", "turn down"][..]);
    m.insert("agree", &["concur", "consent", "accept", "approve", "comply"][..]);
    m.insert("disagree", &["differ", "oppose", "dispute", "contest", "object"][..]);
    m.insert("approve", &["accept", "authorize", "endorse", "sanction", "support"][..]);
    m.insert("deny", &["refuse", "reject", "contradict", "dispute", "decline"][..]);
    m.insert("confirm", &["verify", "validate", "affirm", "establish", "prove"][..]);
    m.insert("prove", &["demonstrate", "show", "verify", "confirm", "establish"][..]);
    m.insert("demonstrate", &["show", "prove", "display", "illustrate", "exhibit"][..]);
    m.insert("reveal", &["show", "disclose", "expose", "uncover", "display"][..]);
    m.insert("hide", &["conceal", "cover", "disguise", "mask", "obscure"][..]);
    m.insert("express", &["convey", "communicate", "state", "voice", "articulate"][..]);
    m.insert("communicate", &["convey", "express", "transmit", "share", "relay"][..]);
    m.insert("report", &["describe", "announce", "state", "document", "present"][..]);
    m.insert("announce", &["declare", "proclaim", "state", "report", "reveal"][..]);
    m.insert("publish", &["release", "issue", "distribute", "print", "announce"][..]);
    m.insert("write", &["compose", "author", "draft", "pen", "record"][..]);
    m.insert("read", &["peruse", "study", "review", "examine", "scan"][..]);
    m.insert("speak", &["talk", "say", "communicate", "express", "articulate"][..]);
    m.insert("listen", &["hear", "attend", "pay attention", "heed"][..]);
    m.insert("learn", &["study", "discover", "understand", "acquire", "master"][..]);
    m.insert("teach", &["instruct", "educate", "train", "show", "guide"][..]);
    m.insert("practice", &["exercise", "train", "rehearse", "apply", "perform"][..]);
    m.insert("apply", &["use", "employ", "implement", "utilize", "exercise"][..]);
    m.insert("serve", &["help", "assist", "aid", "function", "work"][..]);
    m.insert("manage", &["handle", "control", "direct", "run", "administer"][..]);
    m.insert("control", &["manage", "direct", "regulate", "govern", "command"][..]);
    m.insert("direct", &["guide", "lead", "manage", "control", "steer"][..]);
    m.insert("organize", &["arrange", "coordinate", "structure", "plan", "systematize"][..]);
    m.insert("focus", &["concentrate", "center", "emphasize", "target", "direct"][..]);
    m.insert("emphasize", &["stress", "highlight", "underscore", "accentuate", "focus on"][..]);
    m.insert("address", &["tackle", "deal with", "handle", "approach", "confront"][..]);
    m.insert("solve", &["resolve", "fix", "answer", "settle", "work out"][..]);
    m.insert("handle", &["manage", "deal with", "address", "tackle", "control"][..]);
    m.insert("treat", &["handle", "deal with", "regard", "consider", "address"][..]);
    m.insert("regard", &["consider", "view", "see", "treat", "perceive"][..]);
    m.insert("view", &["see", "regard", "consider", "perceive", "observe"][..]);
    m.insert("perceive", &["see", "notice", "observe", "recognize", "sense"][..]);
    m.insert("feel", &["sense", "experience", "perceive", "believe", "think"][..]);
    m.insert("sense", &["feel", "perceive", "detect", "notice", "realize"][..]);
    m.insert("experience", &["encounter", "undergo", "feel", "face", "know"][..]);
    m.insert("suffer", &["endure", "experience", "undergo", "bear", "sustain"][..]);
    m.insert("benefit", &["gain", "profit", "advantage", "help", "improve"][..]);
    m.insert("gain", &["obtain", "acquire", "achieve", "earn", "win"][..]);
    m.insert("lose", &["misplace", "forfeit", "miss", "sacrifice", "drop"][..]);
    m.insert("win", &["gain", "achieve", "earn", "secure", "obtain"][..]);
    m.insert("compete", &["contend", "rival", "vie", "challenge", "contest"][..]);
    m.insert("perform", &["do", "execute", "carry out", "accomplish", "conduct"][..]);
    m.insert("conclude", &["end", "finish", "complete", "determine", "deduce"][..]);
    m.insert("close", &["shut", "end", "conclude", "finish", "seal"][..]);
    m.insert("enable", &["allow", "permit", "empower", "facilitate", "authorize"][..]);
    m.insert("limit", &["restrict", "confine", "constrain", "cap", "curb"][..]);
    m.insert("restrict", &["limit", "confine", "constrain", "control", "regulate"][..]);
    m.insert("promote", &["encourage", "advance", "support", "foster", "boost"][..]);
    m.insert("encourage", &["promote", "support", "motivate", "inspire", "foster"][..]);
    m.insert("discourage", &["deter", "dissuade", "prevent", "inhibit", "demotivate"][..]);
    m.insert("force", &["compel", "make", "pressure", "oblige", "drive"][..]);
    m.insert("influence", &["affect", "impact", "shape", "sway", "determine"][..]);
    m.insert("inspire", &["motivate", "encourage", "stimulate", "influence", "move"][..]);
    m.insert("motivate", &["inspire", "encourage", "drive", "stimulate", "prompt"][..]);
    m.insert("push", &["press", "urge", "drive", "force", "propel"][..]);
    m.insert("pull", &["draw", "drag", "attract", "tug", "extract"][..]);
    m.insert("carry", &["transport", "convey", "bear", "bring", "hold"][..]);
    m.insert("hold", &["grasp", "grip", "keep", "contain", "maintain"][..]);
    m.insert("pick", &["choose", "select", "gather", "collect", "take"][..]);
    m.insert("drop", &["release", "let go", "fall", "decrease", "abandon"][..]);
    m.insert("lift", &["raise", "elevate", "pick up", "hoist", "boost"][..]);
    m.insert("throw", &["toss", "hurl", "pitch", "cast", "fling"][..]);
    m.insert("catch", &["grab", "capture", "seize", "intercept", "snag"][..]);
    m.insert("hit", &["strike", "impact", "beat", "punch", "reach"][..]);
    m.insert("break", &["shatter", "fracture", "crack", "damage", "violate"][..]);
    m.insert("cut", &["slice", "trim", "reduce", "sever", "divide"][..]);
    m.insert("fit", &["suit", "match", "conform", "adapt", "belong"][..]);
    m.insert("fill", &["load", "pack", "occupy", "stuff", "complete"][..]);
    m.insert("cover", &["include", "protect", "hide", "span", "address"][..]);
    m.insert("contain", &["hold", "include", "comprise", "encompass", "incorporate"][..]);
    m.insert("represent", &["symbolize", "depict", "portray", "stand for", "embody"][..]);
    m.insert("reflect", &["show", "mirror", "display", "indicate", "demonstrate"][..]);
    m.insert("indicate", &["show", "suggest", "point to", "signal", "imply"][..]);
    m.insert("imply", &["suggest", "indicate", "hint", "infer", "mean"][..]);
    m.insert("mean", &["signify", "indicate", "intend", "denote", "represent"][..]);
    m.insert("define", &["describe", "explain", "specify", "characterize", "determine"][..]);
    m.insert("relate", &["connect", "link", "associate", "pertain", "concern"][..]);
    m.insert("associate", &["connect", "link", "relate", "combine", "correlate"][..]);
    m.insert("belong", &["fit", "relate", "pertain", "go with"][..]);
    m.insert("depend", &["rely", "hinge", "rest", "count on"][..]);
    m.insert("rely", &["depend", "count on", "trust", "lean on"][..]);
    m.insert("trust", &["believe", "rely on", "depend on", "have faith in"][..]);
    m.insert("doubt", &["question", "distrust", "suspect", "wonder"][..]);
    m.insert("question", &["ask", "doubt", "challenge", "query", "inquire"][..]);
    m.insert("wonder", &["question", "ponder", "speculate", "think about"][..]);
    m.insert("worry", &["concern", "bother", "trouble", "distress", "fret"][..]);
    m.insert("care", &["mind", "worry", "concern oneself", "attend to"][..]);
    m.insert("concern", &["worry", "involve", "affect", "relate to", "matter to"][..]);
    m.insert("matter", &["count", "signify", "be important", "concern"][..]);
    m.insert("count", &["matter", "include", "calculate", "number", "reckon"][..]);
    m.insert("estimate", &["calculate", "assess", "approximate", "gauge", "judge"][..]);
    m.insert("calculate", &["compute", "figure", "determine", "estimate", "work out"][..]);
    m.insert("assess", &["evaluate", "judge", "estimate", "measure", "appraise"][..]);
    m.insert("evaluate", &["assess", "judge", "appraise", "analyze", "examine"][..]);
    m.insert("analyze", &["examine", "study", "evaluate", "investigate", "assess"][..]);
    m.insert("investigate", &["examine", "study", "research", "explore", "probe"][..]);
    m.insert("explore", &["investigate", "examine", "study", "research", "discover"][..]);
    m.insert("review", &["examine", "assess", "evaluate", "analyze", "study"][..]);
    m.insert("observe", &["watch", "notice", "see", "perceive", "monitor"][..]);
    m.insert("monitor", &["watch", "observe", "track", "check", "supervise"][..]);
    m.insert("track", &["follow", "monitor", "trace", "pursue", "record"][..]);
    m.insert("record", &["document", "note", "register", "log", "write down"][..]);
    m.insert("note", &["observe", "notice", "record", "mention", "remark"][..]);
    m.insert("mention", &["note", "refer to", "cite", "state", "bring up"][..]);
    m.insert("refer", &["mention", "allude", "relate", "direct", "cite"][..]);
    m.insert("cite", &["quote", "mention", "refer to", "reference", "name"][..]);
    m.insert("quote", &["cite", "repeat", "reference", "recite"][..]);
    m.insert("state", &["say", "declare", "express", "assert", "announce"][..]);
    m.insert("declare", &["state", "announce", "proclaim", "assert", "affirm"][..]);
    m.insert("assert", &["state", "claim", "declare", "maintain", "affirm"][..]);
    m.insert("insist", &["assert", "maintain", "demand", "claim", "emphasize"][..]);
    m.insert("demand", &["require", "insist", "request", "call for", "need"][..]);
    m.insert("request", &["ask", "demand", "seek", "appeal", "petition"][..]);
    m.insert("offer", &["provide", "give", "present", "propose", "suggest"][..]);
    m.insert("propose", &["suggest", "offer", "recommend", "put forward", "present"][..]);
    m.insert("introduce", &["present", "bring in", "launch", "start", "initiate"][..]);
    m.insert("launch", &["start", "begin", "introduce", "initiate", "release"][..]);
    m.insert("release", &["free", "issue", "publish", "launch", "discharge"][..]);

    // More nouns
    m.insert("world", &["globe", "earth", "planet", "society", "realm"][..]);
    m.insert("country", &["nation", "state", "land", "territory", "homeland"][..]);
    m.insert("government", &["administration", "authority", "regime", "state", "leadership"][..]);
    m.insert("system", &["structure", "arrangement", "method", "framework", "organization"][..]);
    m.insert("process", &["procedure", "method", "operation", "course", "progression"][..]);
    m.insert("case", &["instance", "example", "situation", "matter", "occurrence"][..]);
    m.insert("situation", &["circumstance", "condition", "state", "position", "context"][..]);
    m.insert("condition", &["state", "situation", "circumstance", "status", "requirement"][..]);
    m.insert("position", &["place", "location", "situation", "stance", "role"][..]);
    m.insert("role", &["function", "position", "part", "duty", "responsibility"][..]);
    m.insert("function", &["role", "purpose", "duty", "task", "operation"][..]);
    m.insert("purpose", &["goal", "aim", "objective", "intention", "function"][..]);
    m.insert("goal", &["aim", "objective", "target", "purpose", "ambition"][..]);
    m.insert("aim", &["goal", "objective", "purpose", "target", "intention"][..]);
    m.insert("target", &["goal", "objective", "aim", "focus", "mark"][..]);
    m.insert("effort", &["attempt", "endeavor", "exertion", "work", "try"][..]);
    m.insert("action", &["act", "deed", "measure", "step", "activity"][..]);
    m.insert("activity", &["action", "pursuit", "task", "work", "operation"][..]);
    m.insert("event", &["occurrence", "incident", "happening", "occasion", "episode"][..]);
    m.insert("opportunity", &["chance", "possibility", "opening", "prospect", "occasion"][..]);
    m.insert("chance", &["opportunity", "possibility", "likelihood", "probability", "risk"][..]);
    m.insert("risk", &["danger", "hazard", "chance", "threat", "possibility"][..]);
    m.insert("danger", &["risk", "threat", "hazard", "peril", "menace"][..]);
    m.insert("threat", &["danger", "risk", "menace", "hazard", "warning"][..]);
    m.insert("challenge", &["difficulty", "obstacle", "problem", "test", "task"][..]);
    m.insert("difficulty", &["challenge", "problem", "obstacle", "hardship", "trouble"][..]);
    m.insert("obstacle", &["barrier", "hindrance", "challenge", "impediment", "hurdle"][..]);
    m.insert("solution", &["answer", "resolution", "remedy", "fix", "response"][..]);
    m.insert("response", &["answer", "reply", "reaction", "feedback"][..]);
    m.insert("reaction", &["response", "reply", "feedback", "answer"][..]);
    m.insert("opinion", &["view", "belief", "perspective", "judgment", "stance"][..]);
    m.insert("perspective", &["viewpoint", "outlook", "angle", "position", "standpoint"][..]);
    m.insert("approach", &["method", "way", "strategy", "technique", "angle"][..]);
    m.insert("method", &["approach", "way", "technique", "procedure", "system"][..]);
    m.insert("technique", &["method", "approach", "skill", "procedure", "way"][..]);
    m.insert("strategy", &["plan", "approach", "tactic", "method", "scheme"][..]);
    m.insert("program", &["plan", "scheme", "project", "initiative", "system"][..]);
    m.insert("project", &["plan", "scheme", "undertaking", "venture", "program"][..]);
    m.insert("policy", &["plan", "strategy", "guideline", "rule", "approach"][..]);
    m.insert("rule", &["regulation", "law", "guideline", "principle", "standard"][..]);
    m.insert("law", &["rule", "regulation", "statute", "legislation", "act"][..]);
    m.insert("principle", &["rule", "standard", "guideline", "concept", "belief"][..]);
    m.insert("standard", &["norm", "criterion", "benchmark", "level", "measure"][..]);
    m.insert("step", &["measure", "action", "stage", "phase", "move"][..]);
    m.insert("stage", &["phase", "step", "period", "level", "point"][..]);
    m.insert("phase", &["stage", "period", "step", "chapter", "section"][..]);
    m.insert("period", &["time", "era", "phase", "stage", "span"][..]);
    m.insert("moment", &["instant", "second", "time", "point", "occasion"][..]);
    m.insert("instance", &["example", "case", "occurrence", "occasion", "situation"][..]);
    m.insert("factor", &["element", "aspect", "component", "consideration", "influence"][..]);
    m.insert("aspect", &["element", "factor", "feature", "dimension", "facet"][..]);
    m.insert("feature", &["characteristic", "aspect", "quality", "attribute", "trait"][..]);
    m.insert("characteristic", &["feature", "quality", "trait", "attribute", "property"][..]);
    m.insert("quality", &["characteristic", "attribute", "feature", "property", "standard"][..]);
    m.insert("property", &["characteristic", "quality", "attribute", "feature", "asset"][..]);
    m.insert("type", &["kind", "sort", "category", "variety", "form"][..]);
    m.insert("kind", &["type", "sort", "variety", "category", "class"][..]);
    m.insert("sort", &["type", "kind", "variety", "category", "class"][..]);
    m.insert("structure", &["form", "framework", "organization", "arrangement", "system"][..]);
    m.insert("pattern", &["design", "trend", "model", "template", "arrangement"][..]);
    m.insert("model", &["example", "pattern", "template", "design", "prototype"][..]);
    m.insert("trend", &["pattern", "tendency", "direction", "movement", "development"][..]);
    m.insert("development", &["growth", "progress", "advancement", "evolution", "expansion"][..]);
    m.insert("growth", &["development", "expansion", "increase", "progress", "rise"][..]);
    m.insert("progress", &["advancement", "development", "improvement", "growth", "headway"][..]);
    m.insert("improvement", &["enhancement", "advancement", "progress", "development", "upgrade"][..]);
    m.insert("success", &["achievement", "accomplishment", "triumph", "victory", "attainment"][..]);
    m.insert("failure", &["defeat", "collapse", "breakdown", "setback", "disappointment"][..]);
    m.insert("achievement", &["accomplishment", "success", "attainment", "feat", "triumph"][..]);
    m.insert("source", &["origin", "cause", "root", "basis", "foundation"][..]);
    m.insert("origin", &["source", "beginning", "root", "cause", "start"][..]);
    m.insert("basis", &["foundation", "base", "ground", "reason", "principle"][..]);
    m.insert("foundation", &["basis", "base", "ground", "root", "cornerstone"][..]);
    m.insert("base", &["foundation", "basis", "bottom", "ground", "support"][..]);
    m.insert("service", &["help", "assistance", "aid", "work", "duty"][..]);
    m.insert("advantage", &["benefit", "gain", "edge", "asset", "merit"][..]);
    m.insert("value", &["worth", "importance", "merit", "significance", "benefit"][..]);
    m.insert("price", &["cost", "charge", "fee", "rate", "value"][..]);
    m.insert("rate", &["speed", "pace", "level", "price", "ratio"][..]);
    m.insert("speed", &["rate", "pace", "velocity", "rapidity", "quickness"][..]);
    m.insert("power", &["strength", "force", "authority", "influence", "control"][..]);
    m.insert("strength", &["power", "force", "might", "intensity", "potency"][..]);
    m.insert("energy", &["power", "force", "vitality", "vigor", "drive"][..]);
    m.insert("job", &["work", "task", "position", "employment", "occupation"][..]);
    m.insert("task", &["job", "duty", "assignment", "work", "chore"][..]);
    m.insert("duty", &["responsibility", "obligation", "task", "job", "function"][..]);
    m.insert("responsibility", &["duty", "obligation", "accountability", "role", "burden"][..]);
    m.insert("interest", &["concern", "attention", "curiosity", "stake", "involvement"][..]);
    m.insert("attention", &["focus", "concentration", "notice", "awareness", "interest"][..]);
    m.insert("topic", &["subject", "theme", "issue", "matter", "point"][..]);
    m.insert("subject", &["topic", "theme", "matter", "issue", "field"][..]);
    m.insert("theme", &["topic", "subject", "idea", "motif", "concept"][..]);
    m.insert("concept", &["idea", "notion", "theory", "principle", "thought"][..]);
    m.insert("theory", &["concept", "idea", "hypothesis", "principle", "notion"][..]);
    m.insert("knowledge", &["understanding", "awareness", "information", "expertise", "learning"][..]);
    m.insert("understanding", &["comprehension", "knowledge", "awareness", "grasp", "insight"][..]);
    m.insert("awareness", &["knowledge", "understanding", "consciousness", "recognition", "realization"][..]);
    m.insert("ability", &["capability", "capacity", "skill", "talent", "competence"][..]);
    m.insert("skill", &["ability", "talent", "expertise", "competence", "proficiency"][..]);
    m.insert("talent", &["ability", "skill", "gift", "aptitude", "flair"][..]);
    m.insert("capacity", &["ability", "capability", "potential", "volume", "power"][..]);
    m.insert("potential", &["possibility", "capacity", "capability", "promise", "prospect"][..]);
    m.insert("possibility", &["chance", "potential", "option", "opportunity", "prospect"][..]);
    m.insert("option", &["choice", "alternative", "possibility", "selection"][..]);
    m.insert("choice", &["option", "selection", "decision", "alternative", "preference"][..]);
    m.insert("decision", &["choice", "judgment", "determination", "resolution", "conclusion"][..]);
    m.insert("conclusion", &["end", "result", "decision", "judgment", "finding"][..]);
    m.insert("finding", &["discovery", "conclusion", "result", "outcome", "determination"][..]);
    m.insert("outcome", &["result", "consequence", "effect", "conclusion", "end"][..]);
    m.insert("consequence", &["result", "outcome", "effect", "impact", "repercussion"][..]);
    m.insert("meaning", &["significance", "sense", "definition", "interpretation", "importance"][..]);
    m.insert("feeling", &["emotion", "sense", "impression", "sentiment", "intuition"][..]);
    m.insert("emotion", &["feeling", "sentiment", "passion", "mood", "reaction"][..]);
    m.insert("thought", &["idea", "opinion", "notion", "reflection", "consideration"][..]);
    m.insert("mind", &["brain", "intellect", "consciousness", "psyche", "mentality"][..]);
    m.insert("body", &["physique", "form", "figure", "frame", "organism"][..]);
    m.insert("life", &["existence", "living", "lifetime", "being", "experience"][..]);
    m.insert("death", &["end", "demise", "passing", "mortality", "extinction"][..]);
    m.insert("health", &["wellness", "fitness", "condition", "well-being", "vitality"][..]);
    m.insert("society", &["community", "public", "civilization", "population", "culture"][..]);
    m.insert("community", &["society", "group", "neighborhood", "population", "public"][..]);
    m.insert("family", &["household", "relatives", "kin", "clan", "folks"][..]);
    m.insert("member", &["participant", "part", "component", "element", "associate"][..]);
    m.insert("individual", &["person", "human", "being", "character", "personality"][..]);
    m.insert("character", &["personality", "nature", "quality", "trait", "individual"][..]);
    m.insert("nature", &["character", "essence", "quality", "environment", "world"][..]);
    m.insert("environment", &["surroundings", "setting", "atmosphere", "conditions", "habitat"][..]);
    m.insert("space", &["area", "room", "place", "gap", "expanse"][..]);
    m.insert("room", &["space", "chamber", "area", "capacity", "opportunity"][..]);
    m.insert("building", &["structure", "construction", "edifice", "facility", "premises"][..]);
    m.insert("home", &["house", "residence", "dwelling", "household", "abode"][..]);
    m.insert("house", &["home", "building", "residence", "dwelling", "property"][..]);
    m.insert("land", &["ground", "territory", "earth", "property", "terrain"][..]);
    m.insert("ground", &["land", "earth", "floor", "basis", "foundation"][..]);
    m.insert("surface", &["exterior", "face", "top", "outside", "covering"][..]);
    m.insert("side", &["aspect", "edge", "face", "part", "flank"][..]);
    m.insert("beginning", &["start", "origin", "commencement", "onset", "opening"][..]);
    m.insert("middle", &["center", "midpoint", "midst", "core", "heart"][..]);
    m.insert("center", &["middle", "core", "heart", "focus", "hub"][..]);
    m.insert("edge", &["border", "boundary", "margin", "rim", "brink"][..]);
    m.insert("border", &["edge", "boundary", "limit", "margin", "frontier"][..]);
    m.insert("range", &["scope", "extent", "variety", "span", "spectrum"][..]);
    m.insert("scope", &["range", "extent", "reach", "span", "breadth"][..]);
    m.insert("extent", &["degree", "scope", "range", "size", "magnitude"][..]);
    m.insert("degree", &["extent", "level", "amount", "measure", "intensity"][..]);
    m.insert("amount", &["quantity", "number", "sum", "total", "volume"][..]);
    m.insert("size", &["dimension", "magnitude", "extent", "scale", "proportion"][..]);
    m.insert("shape", &["form", "figure", "outline", "structure", "configuration"][..]);
    m.insert("color", &["hue", "shade", "tone", "tint", "pigment"][..]);
    m.insert("sound", &["noise", "tone", "audio", "voice", "resonance"][..]);
    m.insert("voice", &["sound", "tone", "speech", "expression", "say"][..]);
    m.insert("word", &["term", "expression", "phrase", "vocabulary", "language"][..]);
    m.insert("name", &["title", "designation", "label", "term", "identity"][..]);
    m.insert("title", &["name", "heading", "designation", "label", "position"][..]);
    m.insert("term", &["word", "expression", "phrase", "period", "condition"][..]);
    m.insert("phrase", &["expression", "term", "saying", "clause", "idiom"][..]);
    m.insert("sentence", &["statement", "phrase", "declaration", "judgment"][..]);
    m.insert("statement", &["declaration", "assertion", "claim", "remark", "comment"][..]);
    m.insert("message", &["communication", "note", "statement", "announcement", "news"][..]);
    m.insert("news", &["information", "report", "announcement", "update", "tidings"][..]);
    m.insert("account", &["report", "description", "explanation", "record", "statement"][..]);
    m.insert("document", &["record", "file", "paper", "certificate", "report"][..]);
    m.insert("paper", &["document", "article", "essay", "report", "publication"][..]);
    m.insert("article", &["piece", "item", "paper", "essay", "story"][..]);
    m.insert("story", &["account", "narrative", "tale", "report", "article"][..]);
    m.insert("book", &["volume", "publication", "text", "work", "tome"][..]);
    m.insert("text", &["content", "writing", "document", "passage", "material"][..]);
    m.insert("material", &["substance", "matter", "content", "fabric", "stuff"][..]);
    m.insert("content", &["material", "substance", "subject matter", "information"][..]);
    m.insert("image", &["picture", "representation", "impression", "likeness", "perception"][..]);
    m.insert("picture", &["image", "photo", "illustration", "representation", "scene"][..]);
    m.insert("scene", &["view", "setting", "situation", "sight", "picture"][..]);
    m.insert("sight", &["view", "vision", "scene", "spectacle", "appearance"][..]);

    // =========================================================================
    // More adverbs
    // =========================================================================

    m.insert("just", &["simply", "merely", "only", "exactly", "precisely"][..]);
    m.insert("only", &["just", "merely", "solely", "exclusively", "simply"][..]);
    m.insert("simply", &["just", "merely", "only", "easily", "plainly"][..]);
    m.insert("mostly", &["mainly", "primarily", "largely", "chiefly", "predominantly"][..]);
    m.insert("mainly", &["primarily", "chiefly", "mostly", "largely", "principally"][..]);
    m.insert("largely", &["mostly", "mainly", "primarily", "chiefly", "substantially"][..]);
    m.insert("partly", &["partially", "somewhat", "in part", "to some extent"][..]);
    m.insert("entirely", &["completely", "wholly", "fully", "totally", "absolutely"][..]);
    m.insert("almost", &["nearly", "virtually", "practically", "about", "approximately"][..]);
    m.insert("nearly", &["almost", "virtually", "practically", "approximately", "about"][..]);
    m.insert("exactly", &["precisely", "specifically", "accurately", "just", "correctly"][..]);
    m.insert("directly", &["immediately", "straight", "personally", "exactly"][..]);
    m.insert("immediately", &["instantly", "directly", "promptly", "right away", "at once"][..]);
    m.insert("constantly", &["continually", "continuously", "always", "perpetually", "persistently"][..]);
    m.insert("regularly", &["frequently", "routinely", "consistently", "normally", "habitually"][..]);
    m.insert("frequently", &["often", "regularly", "commonly", "repeatedly", "routinely"][..]);
    m.insert("rarely", &["seldom", "infrequently", "hardly ever", "uncommonly"][..]);
    m.insert("hardly", &["barely", "scarcely", "rarely", "just", "only just"][..]);
    m.insert("barely", &["hardly", "scarcely", "just", "only just", "narrowly"][..]);
    m.insert("relatively", &["comparatively", "fairly", "somewhat", "rather", "reasonably"][..]);
    m.insert("fairly", &["quite", "rather", "reasonably", "moderately", "relatively"][..]);
    m.insert("quite", &["fairly", "rather", "pretty", "somewhat", "relatively"][..]);
    m.insert("rather", &["quite", "fairly", "somewhat", "pretty", "relatively"][..]);
    m.insert("extremely", &["very", "highly", "exceptionally", "incredibly", "immensely"][..]);
    m.insert("highly", &["very", "extremely", "greatly", "exceptionally", "remarkably"][..]);
    m.insert("greatly", &["significantly", "considerably", "substantially", "immensely", "enormously"][..]);
    m.insert("significantly", &["considerably", "substantially", "notably", "greatly", "markedly"][..]);
    m.insert("considerably", &["significantly", "substantially", "greatly", "noticeably", "much"][..]);
    m.insert("substantially", &["significantly", "considerably", "largely", "greatly", "materially"][..]);
    m.insert("particularly", &["especially", "specifically", "notably", "exceptionally", "specially"][..]);
    m.insert("specifically", &["particularly", "especially", "precisely", "explicitly", "exactly"][..]);
    m.insert("generally", &["usually", "typically", "normally", "commonly", "broadly"][..]);
    m.insert("typically", &["usually", "generally", "normally", "commonly", "ordinarily"][..]);
    m.insert("normally", &["usually", "typically", "generally", "ordinarily", "regularly"][..]);
    m.insert("certainly", &["definitely", "surely", "undoubtedly", "absolutely", "clearly"][..]);
    m.insert("definitely", &["certainly", "surely", "absolutely", "clearly", "undoubtedly"][..]);
    m.insert("clearly", &["obviously", "evidently", "plainly", "distinctly", "apparently"][..]);
    m.insert("obviously", &["clearly", "evidently", "plainly", "apparently", "manifestly"][..]);
    m.insert("apparently", &["seemingly", "evidently", "clearly", "obviously", "ostensibly"][..]);
    m.insert("possibly", &["perhaps", "maybe", "potentially", "conceivably", "likely"][..]);
    m.insert("perhaps", &["maybe", "possibly", "potentially", "conceivably"][..]);
    m.insert("naturally", &["of course", "obviously", "normally", "inherently", "instinctively"][..]);
    m.insert("easily", &["readily", "simply", "effortlessly", "smoothly", "comfortably"][..]);
    m.insert("well", &["properly", "effectively", "successfully", "adequately", "thoroughly"][..]);
    m.insert("badly", &["poorly", "terribly", "inadequately", "seriously", "severely"][..]);
    m.insert("carefully", &["cautiously", "thoroughly", "attentively", "meticulously", "precisely"][..]);
    m.insert("strongly", &["firmly", "powerfully", "intensely", "vigorously", "forcefully"][..]);
    m.insert("closely", &["carefully", "tightly", "nearly", "intimately", "attentively"][..]);
    m.insert("widely", &["broadly", "extensively", "generally", "commonly", "universally"][..]);
    m.insert("deeply", &["profoundly", "intensely", "thoroughly", "greatly", "seriously"][..]);
    m.insert("seriously", &["gravely", "severely", "genuinely", "earnestly", "critically"][..]);
    m.insert("properly", &["correctly", "appropriately", "rightly", "suitably", "adequately"][..]);
    m.insert("effectively", &["successfully", "efficiently", "productively", "capably"][..]);
    m.insert("successfully", &["effectively", "triumphantly", "prosperously", "favorably"][..]);
    m.insert("together", &["jointly", "collectively", "simultaneously", "mutually", "in unison"][..]);
    m.insert("separately", &["individually", "independently", "apart", "distinctly"][..]);
    m.insert("alone", &["solely", "only", "independently", "by oneself", "singly"][..]);
    m.insert("instead", &["alternatively", "rather", "in place of", "as a substitute"][..]);
    m.insert("otherwise", &["alternatively", "differently", "or else", "if not"][..]);
    m.insert("meanwhile", &["in the meantime", "simultaneously", "at the same time", "concurrently"][..]);
    m.insert("eventually", &["finally", "ultimately", "in the end", "at last", "sooner or later"][..]);
    m.insert("initially", &["at first", "originally", "to begin with", "primarily"][..]);
    m.insert("previously", &["before", "earlier", "formerly", "prior to this", "in the past"][..]);
    m.insert("subsequently", &["afterward", "later", "then", "next", "following that"][..]);
    m.insert("currently", &["presently", "now", "at present", "today", "at this time"][..]);
    m.insert("recently", &["lately", "of late", "not long ago", "just now"][..]);
    m.insert("soon", &["shortly", "before long", "in a moment", "presently", "quickly"][..]);
    m.insert("already", &["previously", "before now", "by now", "even now"][..]);
    m.insert("still", &["yet", "even now", "nevertheless", "nonetheless", "however"][..]);
    m.insert("yet", &["still", "so far", "up to now", "nevertheless", "however"][..]);
    m.insert("even", &["still", "yet", "indeed", "actually", "in fact"][..]);
    m.insert("indeed", &["in fact", "actually", "truly", "certainly", "really"][..]);
    m.insert("thus", &["therefore", "consequently", "hence", "so", "accordingly"][..]);
    m.insert("hence", &["therefore", "thus", "consequently", "so", "accordingly"][..]);

    // =========================================================================
    // Extra words from the old Rust thesaurus not in Python source
    // =========================================================================

    m.insert("about", &["regarding", "concerning", "relating to"][..]);
    m.insert("many", &["numerous", "several", "various"][..]);
    m.insert("some", &["several", "a few", "certain"][..]);
    m.insert("much", &["considerably", "substantially", "significantly"][..]);
    m.insert("enough", &["sufficient", "adequate", "ample"][..]);
    m.insert("seem", &["appear", "look", "sound"][..]);

    m
});

/// Returns a random synonym for the given word, preserving original capitalization.
///
/// Capitalization rules:
/// - `lowercase` input -> `lowercase` synonym
/// - `Capitalized` input -> `Capitalized` synonym
/// - `ALL CAPS` input -> `ALL CAPS` synonym
///
/// Returns `None` if the word has no synonyms in the thesaurus.
/// Never returns the original word as the synonym.
pub fn get_synonym(word: &str, rng: &mut impl Rng) -> Option<String> {
    let lower = word.to_lowercase();
    let synonyms = THESAURUS.get(lower.as_str())?;

    // Filter out synonyms that match the original word (case-insensitive)
    let candidates: Vec<&&str> = synonyms.iter().filter(|s| **s != lower).collect();
    if candidates.is_empty() {
        return None;
    }

    let idx = rng.gen_range(0..candidates.len());
    let synonym = candidates[idx].to_string();

    // Preserve original capitalization
    Some(apply_capitalization(&synonym, word))
}

/// Returns `true` if the word has synonyms in the thesaurus.
pub fn has_synonym(word: &str) -> bool {
    let lower = word.to_lowercase();
    THESAURUS.contains_key(lower.as_str())
}

/// Apply the capitalization pattern from `original` onto `synonym`.
fn apply_capitalization(synonym: &str, original: &str) -> String {
    if original.len() > 1
        && original
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
    {
        // ALL CAPS
        synonym.to_uppercase()
    } else if original.starts_with(|c: char| c.is_uppercase()) {
        // Capitalized first letter
        let mut chars = synonym.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        }
    } else {
        // lowercase (default)
        synonym.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_has_synonym() {
        assert!(has_synonym("good"));
        assert!(has_synonym("Good"));
        assert!(has_synonym("GOOD"));
        assert!(!has_synonym("asdfghjkl"));
    }

    #[test]
    fn test_get_synonym_returns_some() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = get_synonym("good", &mut rng);
        assert!(result.is_some());
        let synonym = result.unwrap();
        assert_ne!(synonym, "good");
    }

    #[test]
    fn test_get_synonym_unknown_word() {
        let mut rng = StdRng::seed_from_u64(42);
        assert!(get_synonym("xyzzyplugh", &mut rng).is_none());
    }

    #[test]
    fn test_preserves_capitalized() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = get_synonym("Good", &mut rng).unwrap();
        assert!(
            result.starts_with(|c: char| c.is_uppercase()),
            "Expected capitalized, got: {}",
            result
        );
    }

    #[test]
    fn test_preserves_all_caps() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = get_synonym("GOOD", &mut rng).unwrap();
        assert_eq!(
            result,
            result.to_uppercase(),
            "Expected ALL CAPS, got: {}",
            result
        );
    }

    #[test]
    fn test_preserves_lowercase() {
        let mut rng = StdRng::seed_from_u64(42);
        let result = get_synonym("good", &mut rng).unwrap();
        assert_eq!(
            result,
            result.to_lowercase(),
            "Expected lowercase, got: {}",
            result
        );
    }

    #[test]
    fn test_never_returns_same_word() {
        let mut rng = StdRng::seed_from_u64(0);
        for _ in 0..100 {
            if let Some(syn) = get_synonym("good", &mut rng) {
                assert_ne!(syn.to_lowercase(), "good");
            }
        }
    }

    #[test]
    fn test_case_insensitive_lookup() {
        let mut rng = StdRng::seed_from_u64(42);
        assert!(get_synonym("IMPORTANT", &mut rng).is_some());
        assert!(get_synonym("Important", &mut rng).is_some());
        assert!(get_synonym("important", &mut rng).is_some());
    }
}
