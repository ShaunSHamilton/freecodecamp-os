/*
 Generated by typeshare 1.13.2
*/

export interface Landing {
	title: string;
	description: string;
	faq_link: string;
	faq_text: string;
}

export interface Client {
	landing: Record<string, Landing>;
}

export interface CompletedLesson {
	project_id: number;
	lesson_id: number;
}

export interface FreeCodeCampConf {
	client: Client;
	version: string;
}

export interface Hint {
	id: number;
	text: string;
}

export type Seed = 
	| { type: "Command", content: {
	runner: string;
	code: string;
}}
	| { type: "File", content: {
	path: string;
	content: string;
}};

export interface Test {
	code: string;
	id: number;
	runner: string;
	text: string;
}

export interface Lesson {
	after_all: Seed[];
	after_each: Seed[];
	before_all: Seed[];
	before_each: Seed[];
	description: string;
	hints: Hint[];
	id: number;
	seeds: Seed[];
	tests: Test[];
}

export interface LessonMarker {
	project_id: number;
	lesson_id: number;
}

export interface Project {
	title: string;
	description: string;
	id: number;
	is_public: boolean;
	lessons: Lesson[];
}

export interface State {
	locale: string;
	completed_lessons: CompletedLesson[];
}

export type TestState = 
	/** Test has not run yet, or was cancelled */
	| { type: "Neutral", content?: undefined }
	/** Test passed */
	| { type: "Passed", content?: undefined }
	/** Test failed with output */
	| { type: "Failed", content: string }
	/** Test is running */
	| { type: "Running", content?: undefined };

