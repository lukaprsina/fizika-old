const fs = require('fs');

let BOOK_COUNT = 0;
let UNSCRAMBLED_MULTICHOICE_COUNT = 0;

let setMetadata = false;
let output = {
    book_count: 0,
    slides: [],
    videos: [],
    metadata: {},
    flags: {},
    variables: [],
    math: [],
    special_gotos: [],
    transitions: [],
    galleries: []
}

function checkRest(rest) {
    if (typeof rest !== 'undefined') {
        throw "Too many arguments";
    }
}

const Slides = {
    addSlide: (config, rest) => {
        checkRest(rest);
        output.slides.push(config)
    },
    addSpecialGoto: (name, callback, rest) => {
        checkRest(rest);
        output.special_gotos.push({ name, callback: callback.toString() })
    },
    addTransition: (name, callback, rest) => {
        checkRest(rest);
        output.transitions.push({ name, callback: callback.toString() })
    },
}

const Flags = {
    setMetadata: (metadata, rest) => {
        checkRest(rest);
        if (setMetadata)
            throw "Metadata already set";

        output.metadata = metadata;
        setMetadata = true;
    },
    set: (key, value, rest) => {
        checkRest(rest);
        output.flags[key] = value;
    },
}

class VideoClass {
    constructor(id, config, rest) {
        checkRest(rest);

        let valid_keys = ["flowplayer", "html5"]
        for (const [key, value] of Object.entries(config)) {
            const index = valid_keys.indexOf(key);

            if (index > -1) { // only splice array when item is found
                valid_keys.splice(index, 1); // 2nd parameter means remove one item only                    
            } else {
                throw "Video has too few parameters"
            }
        }

        if (valid_keys.length !== 0)
            throw "Video has extra config parameters"

        const playlist = config.flowplayer.playlist;
        if (playlist.length !== 1)
            throw "Playlist has multiple or no videos"

        const url = playlist[0].url

        output.videos.push({ id, url });
    }
}

class VirtualBookClass {
    constructor() { }
    static declareBook() {
        output.book_count += 1;
        BOOK_COUNT += 1;
    }
}

const Variables = {
    setAtDocumentReady: (name, callback, rest) => {
        checkRest(rest);
        output.variables.push({ name, callback: callback.toString() })
    }
}

/* 
new Gallery.Gallery('fffb7edd38cbd2dd937ee30a88f8e996', {
    'vertical': false,
    'fullscreen': true,
    'kind': 'imageflow',
    'bigImageList': ['trenje/teza_klade.jpg', 'trenje/trenje_stat_brus_1.jpg', 'trenje/trenje_stat_les_pluta_1.jpg'],
    'flowAspectRatio': 2,
    'align': 'top',
    'labels': true,
    'height': null,
    'width': null,
    'bigImagePopups': {
        '?2': {
            'src': 'trenje/trenje_stat_les_pluta_1.jpg',
            'title': ''
        },
        '?1': {
            'src': 'trenje/trenje_stat_brus_1.jpg',
            'title': ''
        },
        '?0': {
            'src': 'trenje/teza_klade.jpg',
            'title': ''
        }
    },
    'maxCrop': 0.3,
    'size': 250
});
*/

class GalleryClass {
    constructor(id, config, rest) {
        checkRest(rest)
        output.galleries.push({ id, config })
    }
}

const MathElement = {
    addMath: (name, latex, rest) => {
        checkRest(rest);
        output.math.push({ name, latex })
    }
}

class MultiChoiceClass {
    constructor(name, id, config, rest) {
        checkRest(rest)
        let valid_keys = [
            "style",
            "weight",
            "title",
            "buttonsSameHeight",
            "answers",
            "pick",
            "scramble",
            "columns",
        ]

        const assert = (lhs, rhs) => {
            if (lhs !== rhs)
                throw "Assert failed, lhs: " + lhs + ", rhs: " + rhs;
        }

        for (const [key, value] of Object.entries(config)) {
            const index = valid_keys.indexOf(key);

            if (index > -1) { // only splice array when item is found
                valid_keys.splice(index, 1); // 2nd parameter means remove one item only                    
            } else {
                throw "Video has too few parameters"
            }

            switch (key) {
                case "style":
                    assert(value, "radio")
                    break;
                case "weight":
                    assert(value, 1)
                    break;
                case "pick":
                    assert(value, null)
                    break;
                case "scramble":
                    if (!value)
                        UNSCRAMBLED_MULTICHOICE_COUNT += 1;
                    break;
                case "columns":
                    assert(value, 2)
                    break;
            }
        }

        if (valid_keys.length !== 0)
            throw "Multichoice has extra config parameters"
    }
}

class NumericQuestionClass {
    constructor() { }
}

class ScoreListClass {
    constructor() { }
}

const VirtualBookJS = {
    VirtualBook: VirtualBookClass
}

const NumericQuestion = {
    NumericQuestion: NumericQuestionClass,
}

class SingleChoiceClass {
    constructor() { }
}

const SingleChoice = {
    SingleChoice: SingleChoiceClass,
}

const ScoreList = {
    ScoreList: ScoreListClass,
}

const Gallery = {
    Gallery: GalleryClass,
}

const VideoModule = {
    Video: VideoClass
}

const MultiChoice = {
    MultiChoice: MultiChoiceClass,
}
