

export type Sql4Params = [];


export interface ISql4Result {
    the_count: number;
};


export interface ISql4Query {
    params: Sql4Params;
    result: ISql4Result;
};

