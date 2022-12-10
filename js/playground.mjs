import crypto from "crypto";

export function buildUrlWithStandardPort(parsedUrl) {
  let url = "";
  const port = parsedUrl.protocol === "https:" ? ":443" : ":80";

  url += parsedUrl.protocol ? parsedUrl.protocol + "//" : "";
  url += parsedUrl.username;
  url += parsedUrl.password ? ":" + parsedUrl.password : "";
  url += parsedUrl.username || parsedUrl.password ? "@" : "";
  url += parsedUrl.host ? parsedUrl.host + port : "";
  url += parsedUrl.pathname + parsedUrl.search + parsedUrl.hash;

  return url;
}

export function generateHmac(key, content) {
  return crypto
    .createHmac("sha1", key)
    .update(Buffer.from(content, "utf-8"))
    .digest("base64");
}
