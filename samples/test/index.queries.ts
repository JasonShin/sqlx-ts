export interface ISomeQueryParams {
}

export interface ISomeQueryResult {
  id: number;
  testName: boolean;
}

export interface ISomeQueryQuery {
  params: ISomeQueryParams;
  result: ISomeQueryResult;
}

export interface ITestQuery2Params {
}

export interface ITestQuery2Result {
  id: number;
  hmm: boolean;
}

export interface ITestQuery2Query {
  params: ITestQuery2Params;
  result: ITestQuery2Result;
}

export interface ITestQueryInListParams {
}

export interface ITestQueryInListResult {
  testTest: boolean | null;
}

export interface ITestQueryInListQuery {
  params: ITestQueryInListParams;
  result: ITestQueryInListResult;
}
