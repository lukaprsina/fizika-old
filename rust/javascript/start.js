const fs = require('fs');

let Serialize = {
    videos: []
}

const Slides = {
    addSlide: () => { },
    addSpecialGoto: () => { },
    addTransition: () => { },
}

const Flags = {
    setMetadata: () => { },
    set: () => { },
}

class VideoClass {
    constructor(id, obj, rest) {
        Serialize.videos.push(id);
    }
}

const VideoModule = {
    Video: VideoClass
}

class VirtualBookClass {
    constructor() { }
    static declareBook() { }
}

const VirtualBookJS = {
    VirtualBook: VirtualBookClass
}

const Variables = {
    setAtDocumentReady: () => { }
}

const MathElement = {
    addMath: () => { }
}

class MultiChoiceClass {
    constructor() { }
}

const MultiChoice = {
    MultiChoice: MultiChoiceClass,
}

class NumericQuestionClass {
    constructor() { }
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

class ScoreListClass {
    constructor() { }
}

const ScoreList = {
    ScoreList: ScoreListClass,
}

class GalleryClass {
    constructor() { }
}

const Gallery = {
    Gallery: GalleryClass,
}