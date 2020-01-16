type UUID = String;

export type Invitation = {
  id: UUID;
  email: String;
  expires_at: Date;
};

export type User = {
  email: String;
  hash: String;
  created_at: Date;
};

export type SlimUser = {
  email: String;
};

export const DEFAULT_USER: User = {
  email: "",
  hash: "",
  created_at: new Date()
};
