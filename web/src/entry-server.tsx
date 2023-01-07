// import multer from "multer";
import {
  StartServer,
  createHandler,
  renderAsync,
} from "solid-start/entry-server";

// const upload = multer({ dest: "MULTER_UPLOAD" })
// const middleware = upload.array("avatar");    

export default createHandler(
  //   (input) => {
  // return middleware()
  //   },
  renderAsync((event) => <StartServer event={event} />)
);
