

export type SampleQueryParams = [number, number, number];


export interface ISampleQueryResult {
    id: number;
	points: number;
};


export interface ISampleQueryQuery {
    params: SampleQueryParams;
    result: ISampleQueryResult;
};




export type SampleQuery2Params = [Array<number>];


export interface ISampleQuery2Result {
    id: number;
	points: number;
};


export interface ISampleQuery2Query {
    params: SampleQuery2Params;
    result: ISampleQuery2Result;
};

