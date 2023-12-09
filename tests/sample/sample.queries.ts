

export type SampleSelectQueryParams = [number];


export interface ISampleSelectQueryResult {
    points: number;
	someId: number;
};


export interface ISampleSelectQueryQuery {
    params: SampleSelectQueryParams;
    result: ISampleSelectQueryResult;
};




export type SampleInsertQueryParams = [number];


export interface ISampleInsertQueryResult {
    
};


export interface ISampleInsertQueryQuery {
    params: SampleInsertQueryParams;
    result: ISampleInsertQueryResult;
};




export type SampleUpdateQueryParams = [number, number];


export interface ISampleUpdateQueryResult {
    
};


export interface ISampleUpdateQueryQuery {
    params: SampleUpdateQueryParams;
    result: ISampleUpdateQueryResult;
};




export type SampleDeleteQueryParams = [number];


export interface ISampleDeleteQueryResult {
    
};


export interface ISampleDeleteQueryQuery {
    params: SampleDeleteQueryParams;
    result: ISampleDeleteQueryResult;
};

