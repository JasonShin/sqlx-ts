export type AllNumbersParams = [];

export interface IAllNumbersResult {
	abs1: number;
	acos1: number;
	asin1: number;
	atan1: number;
	avg1: number;
	ceil1: number;
	ceiling1: number;
	cos1: number;
	cot1: number;
	count1: number;
	degrees1: number;
	exp1: number;
	floor1: number;
	greatest1: number;
	least1: number;
	ln1: number;
	log1: number;
	log101: number;
	max1: number;
	min1: number;
	mod1: number;
	pi1: number;
	pow1: number;
	pow2: number;
	radians1: number;
	round0: number;
	sign1: number;
	sin1: number;
	sqrt1: number;
	sum1: number;
	tan1: number;
	trunc0: number;
}

export interface IAllNumbersQuery {
	params: AllNumbersParams;
	result: IAllNumbersResult;
}

export type AllStringsParams = [];

export interface IAllStringsResult {
	age1: string;
	ascii1: string;
	bitLength1: number;
	charLength1: string;
	concat1: string;
	currentDate1: string;
	currentTime1: string;
	currentTimestamp1: string;
	datePartDay1: string;
	datePartHour1: string;
	datePartMinute1: string;
	datePartMonth1: string;
	datePartSecond1: string;
	datePartYear1: string;
	dateTruncDay1: string;
	dateTruncHour1: string;
	dateTruncMinute1: string;
	dateTruncMonth1: string;
	dateTruncSecond1: string;
	dateTruncYear1: string;
	extractDay1: Date;
	extractHour1: Date;
	extractMinute1: Date;
	extractMonth1: Date;
	extractSecond1: Date;
	extractYear1: Date;
	jsonArray1: any;
	jsonBuildObject1: any;
	jsonExtractPathText1: string;
	jsonbArray1: any;
	jsonbBuildObject1: any;
	jsonbExtractPathText1: string;
	left1: string;
	length1: string;
	localtime1: string;
	localtimestamp1: string;
	lower1: string;
	ltrim1: string;
	now1: string;
	octetLength1: string;
	position1: number;
	random1: any;
	repeat1: string;
	replace1: string;
	reverse1: string;
	right1: string;
	rtrim1: string;
	sha11: string;
	sha2561: string;
	sha5121: string;
	substring1: string;
	toChar1: string;
	toDate1: string;
	toJson1: any;
	toJsonb1: any;
	toTimestamp1: string;
	trim1: string;
	upper1: string;
}

export interface IAllStringsQuery {
	params: AllStringsParams;
	result: IAllStringsResult;
}

