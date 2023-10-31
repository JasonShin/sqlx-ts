

export type SampleQueryParams = [];


export interface ISampleQueryResult {
    someId: number;
};


export interface ISampleQueryQuery {
    params: SampleQueryParams;
    result: ISampleQueryResult;
};

