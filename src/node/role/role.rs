enum RoleName {
    Leader,
    Follower,
    Candidate,
}

trait Role {

}

struct AbstractRole {
    roleName: RoleName,
    term: i32
}

impl Role for AbstractRole {
}

struct LeaderRole {
   role: AbstractRole,
   roleName: i32,
}

struct FollowerRole {
    role: AbstractRole
}

struct CandidateRole {
    role: AbstractRole,
    votesCount: i32,
}