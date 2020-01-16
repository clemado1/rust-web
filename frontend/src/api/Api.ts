import axios from "axios";
import { User } from "models";

const api = axios.create({
  baseURL: "https://localhost:8088/"
});

export const authHandler = async (
  url: string,
  method: string
): Promise<User> => {
  const response = await fetch(url, {
    method,
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json"
    }
  });

  return await response.json();
};

export const deserializeUserJson = (promUser: Promise<User>): User => {
  const user: User = {
    email: "",
    hash: "",
    created_at: new Date()
  };

  return user;
};

export const retreiveAuth = (url: string, method: string): User => {
  const promUser = authHandler(url, method);
  return deserializeUserJson(promUser);
};

export const authRequest = async (
  url: string,
  method: string,
  bodyParams?: { email: string; password: string }
): Promise<any> => {
  const response = await fetch(url, {
    method,
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json"
    },
    body: bodyParams ? JSON.stringify(bodyParams) : undefined
  });

  return await response.json();
};
