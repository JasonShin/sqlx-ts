

export type SomeInputQueryParams = [[number, string], [number, number, number]];


export interface ISomeInputQueryResult {
    
};


export interface ISomeInputQueryQuery {
    params: SomeInputQueryParams;
    result: ISomeInputQueryResult;
};

