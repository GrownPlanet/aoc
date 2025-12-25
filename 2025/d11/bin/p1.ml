let read_input filename = In_channel.(with_open_text filename input_lines)

let parse_line line =
  match String.split_on_char ':' line with
  | [ f; t ] -> (f, String.split_on_char ' ' t |> List.filter (fun s -> s <> ""))
  | _ -> invalid_arg "invalid input line"

let rec parse tbl lines =
  match lines with
  | h :: r ->
      let f, t = parse_line h in
      Hashtbl.add tbl f t;
      parse tbl r
  | [] -> tbl

let rec count from connections =
  if from = "out" then 1
  else
    let cons = Hashtbl.find connections from in
    List.fold_left (fun acc c -> acc + count c connections) 0 cons

let () =
  read_input "input.txt"
  |> parse (Hashtbl.create 512)
  |> count "you" |> Printf.printf "%d\n"
