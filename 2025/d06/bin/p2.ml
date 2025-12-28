let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let rec transpose list =
  match list with
  | [] :: _ -> []
  | l -> List.(map hd l :: transpose (map tl l))

let split_input inp =
  String.split_on_char ' ' inp |> List.filter (fun s -> s <> "")

let opp_to_func opp =
  match opp with
  | "*" -> ( * )
  | "+" -> ( + )
  | _ -> invalid_arg "invalid opperation"

let begin_opp opp =
  match opp with "*" -> 1 | "+" -> 0 | _ -> invalid_arg "invalid opperation"

let rec split_double_nl ll_acc l_acc list =
  match list with
  | e :: r ->
      if e = "" then split_double_nl (l_acc :: ll_acc) [] r
      else split_double_nl ll_acc (e :: l_acc) r
  | [] -> l_acc :: ll_acc

let parse_num_map num_map =
  List.map
    (fun l ->
      List.filter_map
        (fun ch ->
          if ch = ' ' then None else Some (String.init 1 (fun _ -> ch)))
        l
      |> List.rev |> String.concat "")
    num_map
  |> split_double_nl [] []
  |> List.map (fun l -> List.map int_of_string l)

let rec solve opps nums =
  match (opps, nums) with
  | opp :: ropps, num :: rnums ->
      List.fold_left (opp_to_func opp) (begin_opp opp) num + solve ropps rnums
  | _ -> 0

let () =
  match read_input "input.txt" |> List.rev with
  | opperators :: nums ->
      let num_map =
        List.map (fun s -> List.init (String.length s) (String.get s)) nums
        |> transpose
      in
      let nm = parse_num_map num_map in
      nm |> solve (List.rev (split_input opperators)) |> Printf.printf "%d\n"
  | [] -> invalid_arg "empty input"
