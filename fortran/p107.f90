module constants

    implicit none
    
    integer, parameter :: N_VERTICES = 40, MAX_EDGES = N_VERTICES ** 2
end module constants

module union_find
    use constants
    implicit none
    
    private
    public uf_init, uf_find, uf_union

    integer, dimension(N_VERTICES) :: uf_parent, uf_rank

contains
    subroutine uf_init()
        integer :: i

        do i = 1, N_VERTICES
            uf_parent(i) = i
        end do
        uf_rank(:) = 0
    end

    recursive function uf_find(a) result(p)
        integer, intent(in) :: a
        integer :: p
        
        p = uf_parent(a)
        if (a .ne. p) then
            p = uf_find(p)
            uf_parent(a) = p
        end if
    end

    subroutine uf_union_inner(a, b)
        integer, intent(in) :: a, b

        if (a .eq. b) then
            return
        else if (uf_rank(a) .lt. uf_rank(b)) then
            uf_parent(a) = b
        else if (uf_rank(a) .gt. uf_rank(b)) then
            uf_parent(b) = a
        else
            uf_parent(b) = a
            uf_rank(a) = uf_rank(a) + 1
        end if
    end

    subroutine uf_union(a, b)
        integer, intent(in) :: a, b
        
        call uf_union_inner(uf_find(a), uf_find(b))
    end
end module union_find

module graph
    use constants
    use union_find
    use stdlib_sorting, only: int_size, sort_index
    implicit none

    type edge
        integer :: a, b, w
    end type

    interface operator (.eq.)
        module procedure edge_eq
    end interface

    interface operator (.ne.)
        module procedure edge_ne
    end interface
contains
    elemental function edge_eq(e1, e2)
        type(edge), intent(in) :: e1, e2
        integer :: a1, b1, a2, b2
        logical :: edge_eq

        if (e1%a .le. e1%b) then
            a1 = e1%a
            b1 = e1%b
        else
            a1 = e1%b
            b1 = e1%a
        end if
        if (e2%a .le. e2%b) then
            a2 = e2%a
            b2 = e2%b
        else
            a2 = e2%b
            b2 = e2%a
        end if

        edge_eq = .true.
        edge_eq = edge_eq .and. a1 .eq. a2
        edge_eq = edge_eq .and. b1 .eq. b2
        edge_eq = edge_eq .and. e1%w .eq. e2%w
    end

    elemental function edge_ne(e1, e2)
        type(edge), intent(in) :: e1, e2
        logical :: edge_ne

        edge_ne = .not. edge_eq(e1, e2)
    end

    subroutine print_edges(edges)
        type(edge), dimension(:), allocatable :: edges
        integer :: i

        do i = 1, size(edges)
            print *, edges(i)
        end do
    end

    function test_case_edges() result(edges)
        type(edge), dimension(:), allocatable :: edges
        
        edges = [ &
            edge(1, 3, 12), edge(3, 6, 31), edge(6, 7, 27), edge(7, 5, 11), edge(2, 5, 20), edge(1, 2, 16), &
            edge(1, 4, 21), edge(3, 4, 28), edge(6, 4, 19), edge(7, 4, 23), edge(5, 4, 18), edge(2, 4, 17) &
        ]
    end

    function read_edges(path) result(edges)
        character(*), intent(in) :: path
        type(edge), dimension(:), allocatable :: edges
        character(10 * N_VERTICES) :: line
        character(10), dimension(N_VERTICES) :: parsed_line
        integer :: i, j, w, edge_idx, iostat
        
        allocate(edges(MAX_EDGES))
        edge_idx = 0

        open(9, file = path)
        do i = 1, N_VERTICES
            read(9, "(A)", end = 99) line
            parsed_line(:) = ""
            read(line, *, iostat = iostat) parsed_line
            do j = i, N_VERTICES
                if (parsed_line(j) .eq. "") then
                    exit
                else if (parsed_line(j) .eq. "-") then
                    cycle
                else
                    read(parsed_line(j), *) w
                    edge_idx = edge_idx + 1
                    edges(edge_idx) = edge(i, j, w)
                end if
            end do
        end do
        99 continue
        close(9)
        edges = edges(:edge_idx)
    end

    subroutine sort_edges(edges)
        type(edge), dimension(:), intent(inout) :: edges
        integer, dimension(:), allocatable :: weights
        integer(kind=int_size), dimension(:), allocatable :: index
        integer :: i

        weights = [(edges(i)%w, i = 1, size(edges))]
        allocate(index(size(edges)))
        call sort_index(weights, index)
        edges(:) = edges(index)
    end

    pure function edge_weight_sum(edges)
        type(edge), dimension(:), intent(in) :: edges
        integer :: edge_weight_sum, i
        
        edge_weight_sum = 0
        do i = 1, size(edges)
            edge_weight_sum = edge_weight_sum + edges(i)%w
        end do
    end

    function kruskall(edges) result(mst)
        type(edge), dimension(:), intent(in) :: edges
        type(edge), dimension(:), allocatable :: mst
        integer :: i, mst_idx

        allocate(mst, mold=edges)
        mst_idx = 0
        call uf_init()

        do i = 1, size(edges)
            if (uf_find(edges(i)%a) .ne. uf_find(edges(i)%b)) then
                call uf_union(edges(i)%a, edges(i)%b)
                mst_idx = mst_idx + 1
                mst(mst_idx) = edges(i)
            end if
        end do
        mst = mst(:mst_idx)
    end

end module graph

program p107
    use graph
    implicit none
    
    type(edge), dimension(:), allocatable :: edges, edges1
    integer :: s, s1

    print "(A)", "Example case"
    edges = read_edges("p107_test.txt")
    call sort_edges(edges)
    edges1 = test_case_edges()
    call sort_edges(edges1)
    if (any(edges .ne. edges1)) error stop "fail"
    call print_edges(edges)

    print "(A)", "Calculating MST"
    edges1 = kruskall(edges)
    call print_edges(edges1)
    
    print "(A)", "MST costs"
    s = edge_weight_sum(edges)
    s1 = edge_weight_sum(edges1)
    print *, s, s1, s - s1

    print "(A)", "Solution"
    edges = read_edges("p107_network.txt")
    call sort_edges(edges)
    edges1 = kruskall(edges)
    s = edge_weight_sum(edges)
    s1 = edge_weight_sum(edges1)
    print *, s, s1, s - s1
end program p107
