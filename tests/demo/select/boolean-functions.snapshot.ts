

export type Sql1Params = [number];


export interface ISql1Result {
    exists: boolean;
};


export interface ISql1Query {
    params: Sql1Params;
    result: ISql1Result;
};




export type Sql16Params = [];


export interface ISql16Result {
    isTrue: boolean;
};


export interface ISql16Query {
    params: Sql16Params;
    result: ISql16Result;
};




export type Sql17Params = [];


export interface ISql17Result {
    isFalse: boolean;
};


export interface ISql17Query {
    params: Sql17Params;
    result: ISql17Result;
};


