const adjectives = [
  "majestic",
  "fierce",
  "cuddly",
  "graceful",
  "working",
  "sleek",
  "wild",
  "sly",
  "persistent",
  "regal",
  "elegant",
  "patient",
  "dignified",
  "intelligent",
  "curious",
  "swift",
  "mighty",
  "commanding",
  "soaring",
  "ferocious",
  "playful",
  "tenacious",
  "elusive",
  "quirky",
  "solitary",
  "gregarious",
  "cunning",
  "versatile",
  "observant",
  "independent",
  "resilient",
  "fearless",
  "adorable",
  "harmless",
  "swiftfooted",
  "eager",
  "nimble",
  "stealthy",
  "furry",
  "feathered",
  "aquatic",
  "sharptoothed",
  "slippery",
  "clumsy",
  "feral",
  "nocturnal",
  "diurnal",
  "exotic",
  "vibrant",
  "sturdy",
  "domestic",
  "experienced",
  "tenacious",
  "hardy",
  "nervous",
  "grumpy",
  "bashful",
  "aloof",
  "courageous",
  "defiant",
  "loyal",
  "sociable",
  "proud",
  "intrepid",
  "adaptable",
  "industrious",
  "patient",
  "persistent",
  "mysterious",
  "magnificent",
  "beautiful",
  "enchanting",
  "enigmatic",
  "romantic",
  "sensual",
  "mystical",
  "fascinating",
  "captivating",
  "charismatic",
  "hypnotic",
  "surreal",
  "otherworldly",
  "fantastic",
  "magical",
  "mythical",
  "legendary",
  "ethereal",
  "enchanting",
  "enlightened",
  "enraged",
  "futuristic",
  "medieval",
  "prehistoric",
  "primordial",
  "shimmering",
  "sinister",
  "spectral",
  "timeless",
  "unearthly",
  "unpredictable",
  "velvety",
  "voluptuous",
  "wondrous",
  "zany"
];

const animals = [ 
  "lynx",
  "wolf",
  "bison",
  "deer",
  "beaver",
  "polecat",
  "boar",
  "elk",
  "fox",
  "badger",
  "eagle",
  "stork",
  "crane",
  "heron",
  "cormorant",
  "swan",
  "raven",
  "magpie",
  "buzzard",
  "kite",
  "osprey",
  "kestrel",
  "otter",
  "marten",
  "wildcat",
  "reindeer",
  "weasel",
  "stoat",
  "hare",
  "rabbit",
  "vole",
  "mole",
  "hedgehog",
  "squirrel",
  "chipmunk",
  "moose",
  "ferret",
  "dormouse",
  "shrew",
  "puma",
  "lynx",
  "roebuck",
  "mouflon",
  "porcupine",
  "polecat",
  "seal",
  "pike",
  "trout",
  "salmon",
  "eel",
  "perch",
  "cod",
  "halibut",
  "lamprey",
  "minnow",
  "catfish",
  "carp",
  "paddlefish",
  "burbot",
  "bream",
  "tench",
  "chub",
  "grayling",
  "gudgeon",
  "ruffe",
  "zander",
  "barbel",
  "bleak",
  "nase",
  "vendace",
  "asp",
  "smelt",
  "catfish",
  "huchen",
  "dace",
  "snake",
  "adder",
  "viper",
  "newt",
  "toad",
  "frog",
  "lynx",
  "bear"
];

function pick(a) {
  return a[Math.floor(Math.random()*a.length)];   
}

function load_name() {
    return pick(adjectives) + "_" + pick(animals) + "_" + Math.floor(2023*Math.random())
}

document.getElementById('inbox').placeholder = load_name()

function go_to_inbox() {
  const inbox = document.getElementById('inbox');
  window.location.href= './inbox.html?user=' + (inbox.value || inbox.placeholder)
}

document.getElementById('inbox_button').onclick = go_to_inbox;
