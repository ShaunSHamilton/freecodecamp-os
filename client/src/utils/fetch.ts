import { FreeCodeCampConf, Lesson, Project, State } from "../types";

interface GetProjectArgs {
  project_id: number;
}

export async function getProject({ project_id }: GetProjectArgs) {
  const res = await fetch(`/projects/${project_id}`);
  const project: Project = await res.json();

  return project;
}

export async function getProjects() {
  const res = await fetch(`/projects`);
  const projects: Project[] = await res.json();

  return projects;
}

export async function getConfig() {
  const res = await fetch(`/config`);
  const config: FreeCodeCampConf = await res.json();

  return config;
}

interface GetLessonArgs {
  project_id: number;
  lesson_id: number;
}

export async function getLesson({ project_id, lesson_id }: GetLessonArgs) {
  const res = await fetch(`/projects/${project_id}/lessons/${lesson_id}`);
  const lesson: Lesson = await res.json();

  return lesson;
}

export async function getState() {
  const res = await fetch(`/state`);
  const state: State = await res.json();

  return state;
}
